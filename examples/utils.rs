use std::collections::HashSet;

fn main() {
    let ss0 = vec!["hr".to_owned(), "employeesNest".to_owned(), "id".to_owned()];
    let ss1 = vec![
        "hr".to_owned(),
        "employeesNest".to_owned(),
        "name".to_owned(),
    ];
    let ss2 = vec![
        "hr".to_owned(),
        "employeesNest".to_owned(),
        "projects".to_owned(),
        "name".to_owned(),
    ];

    let sss_org = vec![ss0, ss1, ss2];

    let mut sss = sss_org.clone();
    let mut common = Vec::<String>::new();
    'a: loop {
        let mut set = HashSet::<String>::new();
        let mut vec = Vec::<Vec<String>>::new();
        for ss in sss.into_iter() {
            if let Some((first, tail)) = ss.split_first() {
                set.insert(first.to_string());
                vec.push(tail.to_vec());
            } else {
                break 'a;
            }
            if set.len() > 1 {
                break 'a;
            };
        }
        if let Some(s) = set.iter().next() {
            common.push(s.to_string());
        } else {
            break 'a;
        }
        sss = vec;
    }
    // dbg!(common);

    let n = common.len();
    let rest_sss = sss_org
        .into_iter()
        .map(|ss| {
            let (_, right) = ss.split_at(n);
            right.to_owned()
        })
        .collect::<Vec<_>>();
    dbg!(&common, &rest_sss);
}
