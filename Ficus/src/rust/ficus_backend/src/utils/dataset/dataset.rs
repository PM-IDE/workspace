use crate::utils::colors::Color;

pub struct FicusDataset {
    values: Vec<Vec<f64>>,
    columns_names: Vec<String>,
    row_names: Vec<String>,
}

impl FicusDataset {
    pub fn new(values: Vec<Vec<f64>>, columns_names: Vec<String>, row_names: Vec<String>) -> Self {
        Self {
            values,
            columns_names,
            row_names,
        }
    }

    pub fn values(&self) -> &Vec<Vec<f64>> {
        &self.values
    }

    pub fn columns_names(&self) -> &Vec<String> {
        &self.columns_names
    }

    pub fn row_names(&self) -> &Vec<String> {
        &self.row_names
    }
}

pub struct LabeledDataset {
    dataset: FicusDataset,
    labels: Vec<usize>,
    labels_colors: Vec<Color>,
}

impl LabeledDataset {
    pub fn new(dataset: FicusDataset, labels: Vec<usize>, labels_colors: Vec<Color>) -> Self {
        Self {
            dataset,
            labels,
            labels_colors,
        }
    }

    pub fn dataset(&self) -> &FicusDataset {
        &self.dataset
    }

    pub fn labels(&self) -> &Vec<usize> {
        &self.labels
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.labels_colors
    }
}
