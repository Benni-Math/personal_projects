use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        // A network with just one layer isn't useful
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();
        
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();
        
        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons 
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
        // Notice: Iterator::size_hint() allows for pre-allocation here -- rustc is smart!
    }
}

/* Within this propagate, there is the implicit assumption that
 * inputs.len() == self.weights.len(), but what if this fails?
 * We could use `thiserror`:
 *  
 *  pub type Result<T> = std::result::Result<T, Error>;
 *  
 *  pub enum Error {
 *      #[error(
 *          "got {got} inputs, but {expected} inputs were expected"
 *      )]
 *      MismatchedInputSize {
 *          got: usize,
 *          expected: usize,
 *      }
 *  }
 * 
 *  ...
 * 
 *  fn propagate(&self, inputs: &[f32]) -> Result<f32> {
 *      if inputs.len() != self.weights.len() {
 *          return Err(Error::MismatchedInputSize {
 *              got: inputs.len(),
 *              expected: self.weights.len(),
 *          })
 *      }
 *      
 *      ...
 * 
 *  }
 * 
 * But, in our case, it's probably better to just panic...
 */
impl Neuron {
    fn random(output_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        
        Self { bias, weights }
    }
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // Checking that the dot product can be done
        assert_eq!(inputs.len(), self.weights.len());

        // using .zip() to dot the vectors -- avoids bound-checks
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        
        // adding bias (affine instead of linear)
        // then applying RELU
        (self.bias + output).max(0.0)
    }
}
