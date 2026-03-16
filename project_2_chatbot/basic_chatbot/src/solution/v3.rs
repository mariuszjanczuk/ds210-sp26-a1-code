use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?

    model: Llama,
    //does not store chat like v2
    user: HashMap<String, Chat<Llama>>, 

    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.

    //dictionary - data linked to user (my comment)
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {

        let chat_memory = model //delete - only create 
        //if user first appears
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        return ChatbotV3 {
        // Make sure you initialize your struct members here
            model: model,
            user: HashMap::new(),
        }; 
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        return String::from("Hello, I am not a bot (yet)!");
    }

//pub fn session(
//    &self,
//) -> Result<impl Deref<Target = <M as CreateChatSession>::ChatSession> + use<'_, M>, &<M as CreateChatSession>::Error>

    #[allow(dead_code)] //std2
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        return Vec::new();
    }
// //Student 2: you need to retrieve the chat session as well. 
// Then, rather than adding a message, you need to retrieve 
// and return the history of the user’s conversation so far 
// as a vector of strings Vec<String>.
//  We suggest you look at the session function from kalosm 
// and its history function. Look at the examples provided by 
// kalosm in the two links above for inspiration. Consider 
// printing the history using println!("{:?}", YOUR HISTORY VARIABLE);.
//  This function’s purpose is to display the history in the 
// UI after a user logs back in (see property 3 in the description 
//  below). 

//!!!!!!!!!!!Thus, this cannot be completely tested without also having your teammates implementation of chat_with_user completed.!!!!!!!!!
}