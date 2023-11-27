#[derive(Debug)]
struct B {}

#[derive(Clone, Debug)]
struct C {}

#[derive(Clone)]
struct D {}

#[derive(Clone, Debug, Sync)]
struct E {}