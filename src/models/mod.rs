trait Model {
    fn insert(&mut self);
    fn update(&mut self);
    fn find_one() -> Self;
}

pub struct User {
    id: u64,
}

impl Model for User {
    fn insert(&mut self) {
        // e.g. send an insert statement to db, update self with db generated id
        self.id = 1;
    }

    fn update(&mut self) {
        // e.g. send an update statement and update a field based on what db returns
    }

    fn find_one() -> User {
        // e.g. run a select statement on db and return only result
        User {id: 1}
    }
}
