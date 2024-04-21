use std::str::FromStr;

use super::mir::ModuledIdentifier;

#[derive(Debug, Clone)]
pub enum Ty {
    Unit,
    Bool,
    I32,
    U32,
    Str,
    Float64,
    Float32,
    SelfDef(ModuledIdentifier),
    Tuple(Vec<Ty>),
    Array(Array),
    Slice(Slice),
    Ref(Box<Ty>),
    Placeholder,
    Dyn(Box<Ty>),
    Result(ModuledIdentifier, Box<Ty>, Box<Ty>),
    Closure(Box<Closure>),
    AllocTy(String, Box<Ty>),
    UND,
    CoroutineClosure(Box<CoroutineClosure>),
    ForeignType(Box<Ty>),
    Mut(Box<Ty>),
    Trait,
    ClosureDefault,
}

impl Default for Ty {
    fn default() -> Self {
        Self::UND
    }
}

impl FromStr for Ty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().nth(0) {
            if c == '&' {
                return Ok(Self::Ref(Box::new(Self::from_str(&s[1..])?)));
            } else if c == '[' {
                return Ok(Self::Array(Array::from_str(s)?));
            }
        }
        if s.contains("Result<") {
            let begin = s.find("Result<").unwrap();
            let mut i = begin + 6;
            let mut left_angle = 0;
            for c in s.get(i..s.len()).unwrap().chars() {
                if c == '<' {
                    left_angle += 1;
                } else if c == '>' {
                    left_angle -= 1;
                } else if left_angle == 1 && c == ',' {
                    break;
                }
                i += 1;
            }
            if i == s.len() {
                let s = s.split("::").map(|s| s.to_string()).collect();
                return Ok(Self::SelfDef(s));
            }
            return Ok(Self::Result(
                s.get(0..begin + 6)
                    .unwrap()
                    .split("::")
                    .map(|s| s.to_string())
                    .collect(),
                Box::new(Ty::from_str(s.get(begin + 7..i).unwrap())?),
                Box::new(Ty::from_str(s.get(i + 2..s.len() - 1).unwrap())?),
            ));
        }
        if let Some(dy) = s.get(0..3) {
            if dy == "dyn" {
                return Ok(Self::Dyn(Box::new(Self::from_str(
                    s.get(5..s.len()).unwrap(),
                )?)));
            }
        }
        if let Some(dy) = s.get(0..6) {
            if dy == "extern" {
                return Ok(Self::Dyn(Box::new(Self::from_str(
                    s.get(12..s.len()).unwrap(),
                )?)));
            }
        }
        match s {
            "f32" => Ok(Self::Float32),
            "f64" => Ok(Self::Float64),
            "()" => Ok(Self::Unit),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "bool" => Ok(Self::Bool),
            "undef" => Ok(Self::UND),
            "str" => Ok(Self::Str),
            "_" => Ok(Self::Placeholder),

            _ => {
                let s = s.split("::").map(|s| s.to_string()).collect();
                Ok(Self::SelfDef(s))
            }
        }
    }
}

impl From<ModuledIdentifier> for Ty {
    fn from(value: ModuledIdentifier) -> Self {
        let s: String = value.iter().map(|s| s.clone()).collect();

        if let Ok(t) = Self::from_str(s.as_str()) {
            t
        } else {
            Self::SelfDef(value)
        }
    }
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        match self {
            Ty::ClosureDefault => "closure".to_string(),
            Ty::Trait => "trait".to_string(),
            Ty::Unit => "()".to_string(),
            Ty::Bool => "bool".to_string(),
            Ty::I32 => "i32".to_string(),
            Ty::U32 => "u32".to_string(),
            Ty::Str => "str".to_string(),
            Ty::Float64 => "f64".to_string(),
            Ty::Float32 => "f32".to_string(),
            Ty::SelfDef(m) => m.join("::"),
            Ty::Tuple(tys) => {
                "(".to_string()
                    + tys
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<String>()
                        .as_str()
                    + ")"
            }
            Ty::Array(a) => a.to_string(),
            Ty::Slice(dt) => dt.to_string(),
            Ty::Ref(r) => "&".to_string() + r.to_string().as_str(),
            Ty::Placeholder => "()".to_string(),
            Ty::Result(m, a, b) => {
                m.join("::") + "<" + a.to_string().as_str() + ", " + b.to_string().as_str() + ">"
            }
            Ty::Dyn(t) => format!("dyn {}", t.to_string()),
            Ty::Mut(t) => format!("mut {}", t.to_string()),
            Ty::ForeignType(ft) => format!("extern C {}", ft.to_string()),
            Ty::Closure(fp) => fp.to_string(),
            Ty::CoroutineClosure(po) => po.to_string(),
            Ty::UND => "UND".to_string(),
            Ty::AllocTy(p, t) => format!("{{{}: {}}}", p, t.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elem_ty: Box<Ty>,
    pub len: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Slice {
    pub elem_ty: Box<Ty>,
}

impl FromStr for Array {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s[1..s.len() - 1].split(';').map(|s| s).collect();
        Ok(Self {
            elem_ty: Box::new(Ty::from_str(s.get(0).unwrap())?),
            len: if let Some(s) = s.get(1) {
                Some(u32::from_str(s).unwrap())
            } else {
                None
            },
        })
    }
}

impl ToString for Array {
    fn to_string(&self) -> String {
        if let Some(l) = self.len {
            "[".to_string() + self.elem_ty.to_string().as_str() + ";" + l.to_string().as_str() + "]"
        } else {
            "[".to_string() + self.elem_ty.to_string().as_str() + "]"
        }
    }
}

impl ToString for Slice {
    fn to_string(&self) -> String {
        "[".to_string() + self.elem_ty.to_string().as_str() + "]"
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub closure_ty: ClosureTy,
    pub params: Vec<Ty>,
    pub ret_ty: Ty,
    pub opeartion: ModuledIdentifier,
}

#[derive(Debug, Clone)]
pub struct CoroutineClosure {
    pub life_cycle_parameters: String,
    pub closure: Closure,
}

#[derive(Debug, Clone)]
pub enum ClosureTy {
    Fn,
    FnOnce,
    FnMut,
}

impl FromStr for Closure {
    type Err = String;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl FromStr for ClosureTy {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fn" => Ok(Self::Fn),
            "fn_mut" => Ok(Self::FnMut),
            "fn_once" => Ok(Self::FnOnce),
            _ => unimplemented!(),
        }
    }
}

impl ToString for Closure {
    fn to_string(&self) -> String {
        format!(
            "{}({}) -> {} {{{}}}",
            self.closure_ty.to_string(),
            self.params
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.ret_ty.to_string(),
            self.opeartion.join("::"),
        )
    }
}

impl ToString for ClosureTy {
    fn to_string(&self) -> String {
        match self {
            ClosureTy::Fn => "fn".to_string(),
            ClosureTy::FnOnce => "fn_once".to_string(),
            ClosureTy::FnMut => "fn_mut".to_string(),
        }
    }
}

impl ToString for CoroutineClosure {
    fn to_string(&self) -> String {
        format!(
            "for<{}> {}",
            self.life_cycle_parameters.to_string(),
            self.closure.to_string()
        )
    }
}
