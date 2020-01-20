// pub struct Screen{
//     displayables:Vec<dyn Drawable>
// }
//
// impl Drawable for screen{
//     fn draw(self:&mut Screen, canvas: &mut render::Canvas<video::Window>){
//         //To Do
//     }
// }

/*pub fn say(a:Option<Character>,b:Option<Text>){
    let mut text, name, voice, text_color, sprite, font;
    if a.is_none() && b.is_none{
        return;
    }
    if a.is_some(){
        character = a.unwrap();
        name = character.name.unwrap();
        voice = character.voice.unwrap();
    }
    if b.is_some(){
        text = character;
    } else {
        text = String::empty();
    }

}*/

//add displayable
//code procédural
//add Solid() : méthode par défaut qui renvoie un displayable : Renvoie un rectangle
//xalign ypos sur chaque displayable

//menu prend une liste d'options, et attend que l'utilisateur en choisit UNE
//menu utilise en sous-main

//say est un screen
//un screen peut prendre des arguments
//utiliser des macros ?
//say : who, what (en paramètres) : who : texte, what : text
