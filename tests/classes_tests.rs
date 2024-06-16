#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    #[tokio::test]
    async fn test_get_class_archer() {
        match get_class("archer").await {
            Ok(class_data) => {
                println!("Archer Data: {:?}", class_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_class_warrior() {
        match get_class("warrior").await {
            Ok(class_data) => {
                println!("Warrior Data: {:?}", class_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_class_assassin() {
        match get_class("assassin").await {
            Ok(class_data) => {
                println!("Assassin Data: {:?}", class_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_class_mage() {
        match get_class("mage").await {
            Ok(class_data) => {
                println!("Mage Data: {:?}", class_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_class_shaman() {
        match get_class("shaman").await {
            Ok(class_data) => {
                println!("Shaman Data: {:?}", class_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_classes() {
        match get_classes().await {
            Ok(class_list) => {
                println!("Class List: {:?}", class_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
