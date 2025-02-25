use super::{
    class::ClassModel,
    family::FamilyModel,
    genus::GenusModel,
    kingdom::KingdomModel,
    order::OrderModel,
    phylum::PhylumModel,
    specie::SpecieModel,
    relational_layer::RelationalLayer,
};

#[derive(Clone)]
pub struct ModelProvider {
    pub kingdom: KingdomModel<RelationalLayer>,
    pub phylum: PhylumModel<RelationalLayer>,
    pub class: ClassModel<RelationalLayer>,
    pub order: OrderModel<RelationalLayer>,
    pub family: FamilyModel<RelationalLayer>,
    pub genus: GenusModel<RelationalLayer>,
    pub specie: SpecieModel<RelationalLayer>,
}

impl ModelProvider {
    pub fn new(relational_layer: RelationalLayer) -> Self {
        Self {
            kingdom: KingdomModel::new(relational_layer.clone()),
            phylum: PhylumModel::new(relational_layer.clone()),
            class: ClassModel::new(relational_layer.clone()),
            order: OrderModel::new(relational_layer.clone()),
            family: FamilyModel::new(relational_layer.clone()),
            genus: GenusModel::new(relational_layer.clone()),
            specie: SpecieModel::new(relational_layer.clone()),
        }
    }
}

