
/* ALTERNATE CSR IMPLEMENTATION */
pub enum Privilege {
    U,
    H,
    S,
    M
}

pub enum Access {
    R,
    W,
    RW,
    WPRI,
    WLRL,
    WARL
}

pub struct Field {
    width: u8,
    position: u8,
    access: Access,
    legal: u64,
    value: u64, 
    
    
}

impl Field {
    fn new(width: u8, position: u8, access: Access, legal: u64) -> Self {
        Field {
            width,
            position,
            access,
            legal:legal,
            value: 0
        }
    }

    fn read(&self) -> u64 {
        match self.access {
            Access::RW => return self.value,
            Access::R => return self.value,
            Access::WARL => return self.value & self.legal,
            Access::WLRL => return self.value & self.legal,
            _ => return self.value & self.legal
        }
    }

    fn write(&mut self, value: u64) {
        match self.access {
            Access::W => self.value = (value & ((1 << self.width) - 1) << self.position),
            Access::RW => self.value = (value & ((1 << self.width) - 1) << self.position),
            Access::WARL => self.value = (value & ((1 << self.width) - 1) << self.position),
            Access::WLRL => self.value = (value & ((1 << self.width) - 1) << self.position) & self.legal,
            _ => self.value = 0,
        }
        
    }

   
}

pub struct CSR {
    name: String,
    privilege: Privilege,
    fields: Vec<Field>,
    value: u64
}

impl CSR {
    pub fn new_csr(name: String, privilege: Privilege) -> Self {
        CSR {
            name: name,
            privilege: privilege,
            fields: Vec::<Field>::new(),
            value: 0
        }
    }

    pub fn add_field(&mut self, width: u8, position: u8, access: Access, legal: u64) {
        self.fields.push(Field::new(width, position, access, legal));
    }

    pub fn read(& mut self) -> u64 {
        return self.value;
    }

    pub fn write(& mut self, value: u64) {
        self.value = 0;
        for field in self.fields.iter_mut() {
            field.write(value);
            self.value |= field.read() << ((1 << field.width) - 1) << field.position;
        }
    }

    
}

impl std::fmt::Debug for CSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
