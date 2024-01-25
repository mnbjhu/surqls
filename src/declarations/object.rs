use super::field::Field;

#[derive(Clone, Debug)]
pub struct Object {
    pub fields: Vec<Field>,
}

impl Object {
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl Object {
    pub fn is_assignable_to(&self, other: &Object) -> bool {
        for field in &self.fields {
            if let Some(other_field) = other.get_field(&field.name) {
                if !field.ty.is_assignable_to(&other_field.ty) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
