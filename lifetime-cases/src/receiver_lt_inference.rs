use std::collections::HashMap;

type PersonId = u16;

pub struct Persons<'a> {
    id_to_person: HashMap<PersonId, Person<'a>>,
    person_to_id: HashMap<Person<'a>, PersonId>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Person<'a> {
    gender: Gender,
    name: &'a str,
}

impl<'a> Persons<'a> {
    // infer lifetime from receiver with special lifetime notion: '_
    // NOTE: it is compiled, but the lifetime of &self is NOT 'a, but much smaller - during the span of current stack frame
    // or, more strictly, defined by the 1) begin: method call site +  2) end: last usage of that &self ref within the method)
    // !!! BUT the gist is actual returned reference has a lifetime constrained by the lifetime of borrowed value, ie OWNER ITSELF.
    pub fn get_id_by_person<'b, 'c, 'd, 'e>(
        &self,
        person: &'b Person<'a>,
        person2: &'c Person<'a>,
    ) -> Option<&'_ PersonId> {
        self.person_to_id.get(person)
    }
}

pub fn lt_inference_demo() {
    let person1 = Person {
        gender: Gender::Male,
        name: "Ferris",
    };

    let p1Clone = Clone::clone(&person1);

    let person2 = Person {
        gender: Gender::Female,
        name: "Jane",
    };
    let p2Clone = Clone::clone(&person2);


    let mut people = Persons {
        id_to_person: HashMap::new(),
        person_to_id: HashMap::new(),
    };

    people.person_to_id.insert(person1, 1);
    people.person_to_id.insert(person2, 2);

    let result = people.get_id_by_person(&p1Clone, &p2Clone);

    // it works - cannot move out because it is borrowed
    // drop(people);
    let result = result.unwrap();

    println!("Found id: {result}");
}
