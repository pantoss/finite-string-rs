struct Layer
{
    weights: Vec<f64>,
    biases: Vec<f64>
}

impl Layer
{
    pub fn new(weights: Vec<f64>,
            biases: Vec<f64>) -> Self
    {
        return Layer{weights:weights, biases:biases}
    }

    pub fn apply(x: Vec<f64>)
    {
        for 
    }
}