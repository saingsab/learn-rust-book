#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        
        let v2 = v1.iter();

        let total: i32 = v2.sum();

        assert_eq!(total, 6);
        //We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
    }

    // collect() use with map, because iter are lazy
    #[test]
    fn iterator_collect() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]); 
    }

    // Using Closures that Capture Their Environment
    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),  
            },
            Shoe {
                size: 10,
                style: String::from("boot"),  
            },
        ];

        let in_my_size = shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10, 
                    style: String::from("sneaker"),
                }, 
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

}
