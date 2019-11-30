


pub trait Mutator<V,H> {
    fn mutate(&self, genome: &mut V, hyperparameters: &H);
}

