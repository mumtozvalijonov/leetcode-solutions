use std::collections::HashMap;

pub struct Solution;

struct Folder {
    name: String,
    exists: bool,
    children: HashMap<String, Folder>,
}

impl Folder {
    pub fn new(name: String) -> Self {
        Folder {
            name: name.clone(),
            exists: false,
            children: HashMap::new(),
        }
    }

    pub fn create(&mut self) {
        self.exists = true;
    }

    pub fn get_or_create_subfolder(&mut self, name: String) -> &mut Self {
        self.children.entry(name.clone()).or_insert(Self::new(name))
    }
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut root_folder = Folder::new("".to_owned());

        for path in folder {
            let mut cur_folder = &mut root_folder;
            for sub_folder in path.split("/").filter(|s| !s.is_empty()) {
                if cur_folder.exists {
                    break;
                }
                cur_folder = cur_folder.get_or_create_subfolder(sub_folder.to_owned());
            }
            cur_folder.create();
        }
        let result = Self::get_parent_folders(&root_folder, "".to_owned());
        result.iter().map(|x| x.to_string()).collect()
    }

    fn get_parent_folders(folder: &Folder, parent_path: String) -> Vec<String> {
        if folder.exists {
            return vec![parent_path + "/" + &folder.name];
        }
        let mut result = vec![];
        let path = {
            if !folder.name.is_empty() {
                parent_path + "/" + &folder.name
            } else {
                "".to_owned()
            }
        };
        for sub_folder in folder.children.values() {
            result.append(&mut Self::get_parent_folders(sub_folder, path.clone()));
        }
        result
    }
}
