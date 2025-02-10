use std::collections::HashMap;

use crate::data::query_builder::QueryBuilder;

pub struct FetchKingdomBuilder {

}

impl QueryBuilder for FetchKingdomBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("SELECT Kingdom, Superkingdom FROM _kingdom".to_owned(), params)
    }
}

pub struct FetchPhylumBuilder {

}

impl QueryBuilder for FetchPhylumBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
         let params = HashMap::<String, String>::new();

        ("SELECT Kingdom, Phylum, Subkingdom FROM _phylum".to_owned(), params)

    }
}

pub struct FetchClassBuilder {

}

impl QueryBuilder for FetchClassBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("SELECT Phylum, _Class, Subphylum FROM _class".to_owned(), params)    
    }
}

pub struct FetchOrderBuilder {

}

impl QueryBuilder for FetchOrderBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("SELECT _Class, _Order, SubClass, Superorder FROM _order".to_owned(), params)
    }
}

pub struct FetchFamilyBuilder {

}

impl QueryBuilder for FetchFamilyBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("SELECT _Order, Family, Suborder, Superfamily FROM _family".to_owned(), params)
    }
}

pub struct FetchGenusBuilder {

}

impl QueryBuilder for FetchGenusBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("SELECT Kingdom, Phylum, Subkingdom FROM _phylum".to_owned(), params)
    }
}

pub struct FetchSpecieBuilder {

}

impl QueryBuilder for FetchSpecieBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("
        SELECT 
            Genus, SpeciesCode, CommonName, Distribution,
            SpeciesAuthor, SpeciesName, SppRecChangedBy, SppRecChangedDate,
            CAST(SppRecordDate AS CHAR) AS SppRecordDateStr, Subgenus, SubspAuthor, Subspecies,
            ValidSpCode, Published 
        FROM _specie".to_owned(),
        params)
    }
}

