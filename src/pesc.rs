pub struct Pesc {
    m_stack: Vec<f64>,
    s_stack: Vec<f64>,
}

impl Pesc {
    pub fn new() -> Self {
        Self {
            m_stack: Vec::new(),
            s_stack: Vec::new(),
        }
    }

    pub fn print(&self) {
        for i in self.m_stack
            .clone().reverse() {
                print!("[{}] ", i);
        }

        println!();
    }

    pub fn eval(&self) {
    }
}
