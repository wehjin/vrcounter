pub enum Report<Mod, Out> {
    Unchanged,
    Model(Mod),
    Outcome(Out),
}
