use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - visit the neighbors in all 4 directions if possible
   - keep track of min and max coordinates as these two
     points are enough to find the smallest rectangle
     enclosing black Pixels
  - calculate the area
*/

impl Solution {
  pub fn min_area(start: (usize, usize), image: &Vec<String>) -> usize {
    let mut image_processed: Vec<Vec<char>> = vec![];

    /* convert the image to a 2D char array */
    for row in image.iter() {
      let chars = Self::to_char_vec(row);
      let mut row_chars: Vec<char> = vec![];
      for c in chars {
        row_chars.push(c);
      }
      image_processed.push(row_chars);
    }
    println!("converted image to char arrays: {:?}", image_processed);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let min_coordinate = &mut (start.0, start.1);
    let max_coordinate = &mut (start.0, start.1);
    /* start visiting */
    Self::visit(
      start,
      &mut visited,
      &image_processed,
      min_coordinate,
      max_coordinate,
    );

    println!("min:{:?}, max:{:?}", min_coordinate, max_coordinate);

    /* calculate the area */
    (max_coordinate.0 - min_coordinate.0 + 1) * (max_coordinate.1 - min_coordinate.1 + 1)
  }

  fn visit(
    node: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    image_processed: &Vec<Vec<char>>,
    min_coordinate: &mut (usize, usize),
    max_coordinate: &mut (usize, usize),
  ) {
    let (lower_x, upper_x, lower_y, upper_y) = (
      0,
      image_processed.len() - 1,
      0,
      image_processed[0].len() - 1,
    );

    let (x, y) = node;
    //visit up
    if x > lower_x {
      let next_node = (x - 1, y);
      if !visited.contains(&next_node) && image_processed[next_node.0][next_node.1] == '1' {
        visited.insert(next_node);
        /* check if we need to increase the rectangle */
        if next_node.0 < min_coordinate.0 {
          min_coordinate.0 = next_node.0;
        }
        println!(
          "visiting node:{:?}, min:{:?}, max:{:?}, visited:{:?}",
          next_node, min_coordinate, max_coordinate, visited
        );

        Self::visit(
          next_node,
          visited,
          image_processed,
          min_coordinate,
          max_coordinate,
        );
      }
    }
    //visit down
    if x < upper_x {
      let next_node = (x + 1, y);
      if !visited.contains(&next_node) && image_processed[next_node.0][next_node.1] == '1' {
        visited.insert(next_node);
        if next_node.0 > max_coordinate.0 {
          max_coordinate.0 = next_node.0;
        }

        println!(
          "visiting node:{:?}, min:{:?}, max:{:?}, visited:{:?}",
          next_node, min_coordinate, max_coordinate, visited
        );

        Self::visit(
          next_node,
          visited,
          image_processed,
          min_coordinate,
          max_coordinate,
        );
      }
    }

    //visit left
    if y > lower_y {
      let next_node = (x, y - 1);
      if !visited.contains(&next_node) && image_processed[next_node.0][next_node.1] == '1' {
        visited.insert(next_node);
        if next_node.1 < min_coordinate.1 {
          min_coordinate.1 = next_node.1;
        }
        println!(
          "visiting node:{:?}, min:{:?}, max:{:?}, visited:{:?}",
          next_node, min_coordinate, max_coordinate, visited
        );
        Self::visit(
          next_node,
          visited,
          image_processed,
          min_coordinate,
          max_coordinate,
        );
      }
    }

    //visit right
    if y < upper_y {
      let next_node = (x, y + 1);
      if !visited.contains(&next_node) && image_processed[next_node.0][next_node.1] == '1' {
        visited.insert(next_node);
        if next_node.1 > max_coordinate.1 {
          max_coordinate.1 = next_node.1;
        }
        println!(
          "visiting node:{:?}, min:{:?}, max:{:?}, visited:{:?}",
          next_node, min_coordinate, max_coordinate, visited
        );
        Self::visit(
          next_node,
          visited,
          image_processed,
          min_coordinate,
          max_coordinate,
        );
      }
    }
  }

  pub fn to_char_vec(input: &String) -> Vec<char> {
    input.to_ascii_lowercase().chars().collect()
  }

  pub fn test_fixture_1() -> Vec<String> {
    vec![
      String::from("0010"),
      String::from("0110"),
      String::from("0100"),
    ]
  }

  pub fn test_fixture_2() -> Vec<String> {
    vec![
      String::from("0110"),
      String::from("0110"),
      String::from("0100"),
    ]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::min_area((0, 2), &Solution::test_fixture_1());
    assert_eq!(result, 6);
  }

  #[test]
  fn sample_2() {
    let result = Solution::min_area((0, 2), &Solution::test_fixture_2());
    assert_eq!(result, 6);
  }
}
