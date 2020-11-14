#[derive(Debug)]
struct Tensor {
    data: Vec<Vec<f64>>,
    grad: Option<Box<Tensor>>,
    shape: (usize, usize),
}

impl Tensor {
    fn new(data: &Vec<Vec<f64>>) -> Self {
        let shape = (data.len(), data[0].len());

        Self {
            data: data.clone(),
            grad: Some(Box::new(Tensor::zeros(shape))),
            shape: shape
        }
    }

    fn ones(size: (usize, usize)) -> Self {
        let mut data = Vec::new();
        
        for _ in 0..size.0 {
            let mut inner = Vec::new();
            
            for _ in 0..size.1 {
                inner.push(1.0);
            }

            data.push(inner);
        }

        let shape = (data.len(), data[0].len());
        
        Self {
            data: data,
            grad: None,
            shape: shape
        }
    }

    fn zeros(size: (usize, usize)) -> Self {
        let mut data = Vec::new();
        
        for _ in 0..size.0 {
            let mut inner = Vec::new();
            
            for _ in 0..size.1 {
                inner.push(0.0);
            }

            data.push(inner);
        }

        let shape = (data.len(), data[0].len());
        
        Self {
            data: data,
            grad: None,
            shape: shape
        }
    }

    fn dot(&self, x: Tensor) -> Self {
        let mut data = Vec::new();

        if x.shape.0 != self.shape.1 { panic!("{} != {}", x.shape.0, self.shape.1) }

        for i in 0..self.shape.0 {
            let mut inner = Vec::new();
            
            for j in 0..x.shape.1 {
                inner.push(0.0);
                
                for k in 0..x.shape.0 {
                    inner[j] = inner[j] + self.data[i][k] * x.data[k][j];
                }
            }

            data.push(inner);
        }

        let shape = (data.len(), data[0].len());

        Self {
            data: data,
            grad: Some(Box::new(Tensor::zeros(shape))),
            shape: shape
        }
    }

    fn backward(&self) -> Self {
        Self {
            data: vec![vec![]],
            grad: None,
            shape: (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let mut data = Vec::new();
        data.push(vec![1.0, 2.0]);

        let tensor = Tensor::new(&data);

        assert_eq!(tensor.shape, (1, 2));
        assert_eq!(tensor.data, [[1.0, 2.0]]);
        assert_eq!(tensor.grad.unwrap().data, [[0.0, 0.0]]);
    }

    #[test]
    fn test_create_ones() {
        let tensor = Tensor::ones((2, 3));

        assert_eq!(tensor.shape, (2, 3));
        assert_eq!(tensor.data, [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
    }

    #[test]
    fn test_create_zeros() {
        let tensor = Tensor::zeros((2, 3));

        assert_eq!(tensor.shape, (2, 3));
        assert_eq!(tensor.data, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
    }

    #[test]
    fn test_dot_tensors() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![3.0, 2.0, 3.0]];

        let tensor1 = Tensor::new(&data);
        let tensor2 = Tensor::ones((3, 2));
        let tensor3 = tensor1.dot(tensor2);

        assert_eq!(tensor3.shape, (2, 2));
        assert_eq!(tensor3.data, vec![vec![6.0, 6.0], vec![8.0, 8.0]]);
    }
}