struct World {
    // We'll use `entities_count` to assign each Entity a unique ID.
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        self.component_vecs
            .iter_mut()
            .for_each(|component_vec| {
                component_vec.push_none();
            });

        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(c_v) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>() 
            {
                c_v[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet
        let mut new_cv: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);
        
        for _ in 0..self.entities_count {
            new_cv.push(None);
        }

        new_cv[entity] = Some(component);
        self.component_vecs.push(Box::new(new_cv));
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }
}

fn main() {
    todo!()
}