use super::{
    class::ClassModel,
    country::CountryModel,
    family::FamilyModel,
    genus::GenusModel,
    graph_layer::GraphLayer,
    kingdom::KingdomModel,
    order::OrderModel,
    phylum::PhylumModel,
    specie::SpecieModel,
    state::StateModel
};

#[derive(Clone)]
pub struct ModelProvider {
    pub kingdom: KingdomModel<GraphLayer>,
    pub phylum: PhylumModel<GraphLayer>,
    pub class: ClassModel<GraphLayer>,
    pub order: OrderModel<GraphLayer>,
    pub family: FamilyModel<GraphLayer>,
    pub genus: GenusModel<GraphLayer>,
    pub specie: SpecieModel<GraphLayer>,
    pub country: CountryModel<GraphLayer>,
    pub state: StateModel<GraphLayer>,
}

impl ModelProvider {
    pub fn new(graph_layer: GraphLayer) -> Self {
        Self {
            kingdom: KingdomModel::new(graph_layer.clone()),
            phylum: PhylumModel::new(graph_layer.clone()),
            class: ClassModel::new(graph_layer.clone()),
            order: OrderModel::new(graph_layer.clone()),
            family: FamilyModel::new(graph_layer.clone()),
            genus: GenusModel::new(graph_layer.clone()),
            specie: SpecieModel::new(graph_layer.clone()),
            country: CountryModel::new(graph_layer.clone()),
            state: StateModel::new(graph_layer.clone()),
        }
    }
}

