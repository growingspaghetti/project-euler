// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

struct Index {
    i: usize,
    _ite: Box<dyn Iterator<Item = usize>>,
}

impl Index {
    fn increment(&mut self) {
        self.i += self._ite.next().unwrap();
    }
    fn new() -> Self {
        Self {
            i: 5,
            _ite: Box::new(vec![2usize, 4].into_iter().cycle()),
        }
    }
}

struct Sieve {
    _sieve: Vec<bool>,
    _primes: Vec<usize>,
    _index: Index,
    _queue: std::collections::VecDeque<usize>,
}

impl Sieve {
    fn rule_out(&mut self, prime: usize) {
        for i in (prime * prime..self._sieve.len()).step_by(prime) {
            self._sieve[i] = false;
        }
    }
    fn rule_out_from_previous_position(&mut self, prime: usize, pp: usize) {
        for i in (prime * prime..self._sieve.len()).step_by(prime) {
            if i < pp {
                continue;
            }
            self._sieve[i] = false;
        }
    }
    fn clean_sieve(&mut self) {
        let sqrt = (self._sieve.len() as f32).sqrt() as usize;
        while self._index.i <= sqrt {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self._queue.push_back(self._index.i);
                self.rule_out(self._index.i);
            }
            self._index.increment();
        }
        while self._index.i < self._sieve.len() {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self._queue.push_back(self._index.i);
            }
            self._index.increment();
        }
    }
    fn new(below: u32) -> Self {
        assert!(below > 4);
        let sieve = vec![true; below as usize];
        let mut s = Self {
            _sieve: sieve,
            _primes: vec![],
            _index: Index::new(),
            _queue: std::collections::VecDeque::new(),
        };
        s._queue.push_back(2);
        s._queue.push_back(3);
        s.clean_sieve();
        s
    }
    fn extend(&mut self) {
        let previous_len = self._sieve.len();
        self._sieve.extend(vec![true; previous_len]);
        for &p in &self._primes.clone() {
            self.rule_out_from_previous_position(p, previous_len);
        }
        self.clean_sieve();
    }
    fn is_prime(&mut self, n: u32) -> bool {
        if n == 2 || n == 3 {
            return true;
        }
        if n == 0 || n == 1 || n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        while n > self._sieve.len() as u32 - 1 {
            self.extend();
        }
        self._sieve[n as usize]
    }
    fn next_prime(&mut self) -> u32 {
        loop {
            if let Some(p) = self._queue.pop_front() {
                return p as u32;
            }
            self.extend();
        }
    }
}

fn appearance_frequency_excluding_tail(mut p: u32, frequency: &mut Frequency) {
    frequency.clear();
    p /= 10;
    while p > 0 {
        let d = p % 10;
        frequency.0[d as usize] += 1;
        p /= 10;
    }
}

fn expand_wildcard(mut p: u32, wildcard: u8, expansion: &mut WildcardExpansion) {
    //nums: &mut [u32; 10]) {
    expansion.clear();
    let mut dicimal_place = 1u32;
    let first_digit = p % 10;
    for n in &mut expansion.0 {
        *n += first_digit;
    }
    p /= 10;
    while p > 0 {
        dicimal_place *= 10;
        let d = (p % 10) as u8;
        let is_wildcard = d as u8 == wildcard;
        for (i, n) in expansion.0.iter_mut().enumerate() {
            *n += dicimal_place * if is_wildcard { i as u32 } else { d as u32 };
        }
        p /= 10;
    }
}

fn generate_numbers(n: u32, wild_card: u8, sieve: &mut Sieve) -> (u32, u8) {
    // , checked: &mut BTreeMap<u32, bool>
    let mut min = u32::MAX;
    let mut count = 0u8;
    for i in (0u32..=9).rev() {
        let mut t = n.clone();
        let mut m = 1u32;
        let mut replaced = t % 10;
        t /= 10;
        while t > 0 {
            m *= 10;
            let d = (t % 10) as u8;
            replaced += m * if d as u8 == wild_card { i } else { d as u32 };
            t /= 10;
        }

        if replaced / m == 0 {
            //dbg!("-{}", replaced);
            continue;
        }
        if sieve.is_prime(replaced) {
            //checked.insert(replaced, true);
            //dbg!("{}", replaced);
            min = replaced;
            count += 1;
        }

        // append or exclude 0****
    }
    // if count == 8 {
    //     dbg!("p:{}\tc:{}", n, count);
    // }
    (min, count)
}

use std::collections::BTreeMap;

fn digit_len_match(mut a: u32, mut b: u32) -> bool {
    while a > 0 && b > 0 {
        a /= 10;
        b /= 10;
    }
    a == b
}

struct Frequency([u8; 10]);
impl Frequency {
    const BLANK: [u8; 10] = [0u8; 10];
    fn clear(&mut self) {
        //self.0.iter_mut().for_each(|x| *x = 0)
        self.0.copy_from_slice(&Self::BLANK[..]);
    }
    fn new() -> Self {
        Frequency(Frequency::BLANK.clone())
    }
}

struct WildcardExpansion([u32; 10]);
impl WildcardExpansion {
    const BLANK: [u32; 10] = [0u32; 10];
    fn clear(&mut self) {
        //self.0.iter_mut().for_each(|x| *x = 0)
        self.0.copy_from_slice(&Self::BLANK[..]);
    }
    fn new() -> Self {
        WildcardExpansion(WildcardExpansion::BLANK.clone())
    }
}

// 17 ms
/// ```rust
/// use self::project_euler::m51::smallest_prime_which_by_replacing_part;
///// assert_eq!(smallest_prime_which_by_replacing_part(2, 1), 1013);
///// assert_eq!(smallest_prime_which_by_replacing_part(2, 2), 229);
/// assert_eq!(smallest_prime_which_by_replacing_part(1, 6), 13);
/// assert_eq!(smallest_prime_which_by_replacing_part(2, 7), 56003);
/// assert_eq!(smallest_prime_which_by_replacing_part(3, 8), 121313);
/// assert_eq!(smallest_prime_which_by_replacing_part(4, 6), 2422027); // https://www.hackerrank.com/contests/projecteuler/challenges/euler051/forum/comments/310849
/// assert_eq!(smallest_prime_which_by_replacing_part(4, 7), 80047003); // https://www.hackerrank.com/contests/projecteuler/challenges/euler051/forum/comments/592444
/// assert_eq!(smallest_prime_which_by_replacing_part(3, 9), 38000201); // https://projecteuler.net/action=quote;post_id=382609
///// u64 assert_eq!(smallest_prime_which_by_replacing_part(3, 10), 181030208131); // https://projecteuler.net/action=quote;post_id=369509
/// ```
pub fn smallest_prime_which_by_replacing_part(replacement: u8, family_length: u8) -> u32 {
    assert!(family_length > 4);
    let mut sieve = Sieve::new(10_000);
    let mut p = sieve.next_prime();
    //let mut checked = Vec::<u32>::new();
    //let mut checked = BTreeMap::<u32, bool>::new();
    while p < 10 {
        p = sieve.next_prime();
    }

    let mut frequency = Frequency::new();
    let mut wildcard_expansion = WildcardExpansion::new();
    //let mut checked = BinarySearchTree::new();
    //let (blank_freq, mut frequency) = ([0u8; 10], [0u8; 10]);
    // let (blank_nums, mut nums) = ([0u32; 10], [0u32; 10]);
    //let not_found_threshold = 10u32.pow(10 + replace as u32);
    'explorarion: loop {
        // if checked.contains(p) {
        //     p = sieve.next_prime();
        //     continue;
        // }
        // if checked.contains_key(&p) {
        //     p = sieve.next_prime();
        //     continue;
        // }
        //frequency.copy_from_slice(&blank_freq);
        appearance_frequency_excluding_tail(p, &mut frequency);
        for (i, _) in frequency
            .0
            .iter()
            .enumerate()
            .filter(|(_, &f)| f == replacement)
        {
            //nums.copy_from_slice(&blank_nums);
            expand_wildcard(p, i as u8, &mut wildcard_expansion);

            //for &n in
            let mut family = wildcard_expansion.0[1..]
                .iter()
                .map(|&n| n)
                .filter(|&n| sieve.is_prime(n))
                .collect::<Vec<u32>>();
            if sieve.is_prime(wildcard_expansion.0[0]) && digit_len_match(wildcard_expansion.0[0], p) {
                family.insert(0, wildcard_expansion.0[0]);
            }
            // {
            //     if sieve.is_prime(n) {
            //         count += 1;
            //         min = n;
            //     }
            // }
            //let (min, count) = generate_numbers(p, i as u8, &mut sieve); //, &mut checked);
            if family.len() as u8 == family_length {
                //dbg!("{:?}", &family);
                break 'explorarion family[0];
            }
            //family.iter().for_each(|&p| checked.insert(p));
        }
        p = sieve.next_prime();
        //if p > not_found_threshold {
        //    panic!("Not found below {}", not_found_threshold);
        //}
    }
}

use std::cmp::Ordering;

pub struct BinarySearchTree {
    root: OptNode,
}

type OptNode = Option<Box<Node>>;

struct Node {
    key: u32,
    left: OptNode,
    right: OptNode,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn contains(&self, key: u32) -> bool {
        let mut node = &self.root;
        while let Some(ref boxed_node) = *node {
            match key.cmp(&boxed_node.key) {
                Ordering::Less => node = &boxed_node.left,
                Ordering::Greater => node = &boxed_node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }

    pub fn insert(&mut self, key: u32) {
        let mut node = &mut self.root;
        while let Some(ref mut boxed_node) = *node {
            match key.cmp(&boxed_node.key) {
                Ordering::Less => node = &mut boxed_node.left,
                Ordering::Greater => node = &mut boxed_node.right,
                Ordering::Equal => return,
            }
        }
        *node = Some(Box::new(Node {
            key: key,
            left: None,
            right: None,
        }));
    }
}
