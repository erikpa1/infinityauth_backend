use azure_tools::{CONST_PK, CONST_RK};

pub struct LicenceTable {}


impl LicenceTable {
    //This is organization ID
    pub fn PK() -> &'static str {
        return CONST_PK!();
    }
    //This is application ID
    pub fn RK() -> &'static str {
        return CONST_RK!();
    }

    pub fn TYPE() -> &'static str {
        return "Type";
    }

    pub fn UID() -> &'static str {
        return "Uid";
    }

    pub fn VALID_TO() -> &'static str {
        return "ValidTo";
    }
}


pub struct ApplicationTable {}

impl ApplicationTable {
    pub fn PK() -> &'static str {
        return CONST_PK!();
    }
    pub fn RK() -> &'static str {
        return CONST_RK!();
    }

    pub fn NAME() -> &'static str {
        return "Name";
    }
    pub fn TYPE() -> &'static str {
        return "Type";
    }
}

pub struct UserTable {}

impl UserTable {}

pub struct OrganizationTable {}

impl OrganizationTable {
    pub fn PK() -> &'static str {
        return CONST_PK!();
    }

    pub fn RK() -> &'static str {
        return CONST_RK!();
    }

    pub fn NAME() -> &'static str {
        return "Name";
    }
    pub fn IMAGE_PATH() -> &'static str {
        return "ImagePath";
    }
}