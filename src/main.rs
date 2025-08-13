extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const LARGEUR_ECRAN: i32 = 800;
const HAUTEUR_ECRAN: i32 = 800;
const VITESSE_MIN: i32 = 2;
const VITESSE_MAX: i32 = 3;
const LARGEUR_VOITURE: i32 = 20;
const HAUTEUR_VOITURE: i32 = 20;
const DISTANCE_SECURITE: i32 = 30;
const COULEUR_GAUCHE: Color = Color::RGB(255, 0, 0);
const COULEUR_DROITE: Color = Color::RGB(0, 255, 0);
const COULEUR_TOUT_DROIT: Color = Color::RGB(0, 0, 255);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Gauche,
    Droite,
    ToutDroit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cote {
    Sud,
    Nord,
    Ouest,
    Est,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Feu {
    Vert,
    Rouge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeuTricolore {
    pub couleur: Feu,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Voiture {
    pub x: i32,
    pub y: i32,
    pub couleur: Color,
    pub direction: Direction,
    pub cote: Cote,
    pub vitesse: i32,
}

impl Voiture {
    pub fn nouvelle(cote: Cote) -> Voiture {
        let mut rng = rand::thread_rng();
        let nombre_aleatoire = rng.gen_range(0..3);
        let vitesse = rng.gen_range(VITESSE_MIN..VITESSE_MAX);
        let direction: Direction;
        let couleur: Color;
        match nombre_aleatoire {
            0 => {
                direction = Direction::Gauche;
                couleur = COULEUR_GAUCHE;
            }
            1 => {
                direction = Direction::ToutDroit;
                couleur = COULEUR_TOUT_DROIT;
            }
            _ => {
                direction = Direction::Droite;
                couleur = COULEUR_DROITE;
            }
        }
        match cote {
            Cote::Est => {
                let x = 0;
                let y = HAUTEUR_ECRAN / 2;
                Voiture {
                    x,
                    y,
                    couleur,
                    direction,
                    cote,
                    vitesse,
                }
            }
            Cote::Nord => {
                let x = LARGEUR_ECRAN / 2 - LARGEUR_VOITURE;
                let y = 0;
                Voiture {
                    x,
                    y,
                    couleur,
                    direction,
                    cote,
                    vitesse,
                }
            }
            Cote::Sud => {
                let x = LARGEUR_ECRAN / 2;
                let y = HAUTEUR_ECRAN - HAUTEUR_VOITURE;
                Voiture {
                    x,
                    y,
                    couleur,
                    direction,
                    cote,
                    vitesse,
                }
            }
            Cote::Ouest => {
                let x = LARGEUR_ECRAN - LARGEUR_VOITURE;
                let y = HAUTEUR_ECRAN / 2 - HAUTEUR_VOITURE;
                Voiture {
                    x,
                    y,
                    couleur,
                    direction,
                    cote,
                    vitesse,
                }
            }
        }
    }

    pub fn voiture_aleatoire() -> Voiture {
        let mut rng = rand::thread_rng();
        let nombre_aleatoire = rng.gen_range(0..4);
        match nombre_aleatoire {
            0 => Voiture::nouvelle(Cote::Est),
            1 => Voiture::nouvelle(Cote::Nord),
            2 => Voiture::nouvelle(Cote::Sud),
            _ => Voiture::nouvelle(Cote::Ouest),
        }
    }

    pub fn deplacer(&mut self, feu: FeuTricolore) {
        match self.cote {
            Cote::Est => {
                if self.x + self.vitesse < LARGEUR_ECRAN / 2 - 2 * LARGEUR_VOITURE {
                    self.x += self.vitesse;
                } else if self.x + self.vitesse < LARGEUR_ECRAN / 2 - LARGEUR_VOITURE {
                    if feu.couleur == Feu::Vert {
                        self.x += self.vitesse;
                    } else {
                        self.x = LARGEUR_ECRAN / 2 - 2 * LARGEUR_VOITURE;
                    }
                } else {
                    match self.direction {
                        Direction::Gauche => {
                            self.x = LARGEUR_ECRAN / 2;
                            self.y -= self.vitesse;
                        }
                        Direction::Droite => {
                            self.x = LARGEUR_ECRAN / 2 - LARGEUR_VOITURE;
                            self.y += self.vitesse;
                        }
                        Direction::ToutDroit => {
                            self.x += self.vitesse;
                        }
                    }
                }
            }
            Cote::Nord => {
                if self.y + self.vitesse < HAUTEUR_ECRAN / 2 - 2 * HAUTEUR_VOITURE {
                    self.y += self.vitesse;
                } else if self.y + self.vitesse < HAUTEUR_ECRAN / 2 - HAUTEUR_VOITURE {
                    if feu.couleur == Feu::Vert {
                        self.y += self.vitesse;
                    } else {
                        self.y = HAUTEUR_ECRAN / 2 - 2 * HAUTEUR_VOITURE;
                    }
                } else {
                    match self.direction {
                        Direction::Gauche => {
                            self.y = HAUTEUR_ECRAN / 2;
                            self.x += self.vitesse;
                        }
                        Direction::Droite => {
                            self.y = HAUTEUR_ECRAN / 2 - HAUTEUR_VOITURE;
                            self.x -= self.vitesse;
                        }
                        Direction::ToutDroit => {
                            self.y += self.vitesse;
                        }
                    }
                }
            }
            Cote::Ouest => {
                if self.x - self.vitesse > LARGEUR_ECRAN / 2 + 2 * LARGEUR_VOITURE {
                    self.x -= self.vitesse;
                } else if self.x - self.vitesse > LARGEUR_ECRAN / 2 {
                    if feu.couleur == Feu::Vert {
                        self.x -= self.vitesse;
                    } else {
                        self.x = LARGEUR_ECRAN / 2 + LARGEUR_VOITURE;
                    }
                } else {
                    match self.direction {
                        Direction::Gauche => {
                            self.x = LARGEUR_ECRAN / 2 - LARGEUR_VOITURE;
                            self.y += self.vitesse;
                        }
                        Direction::Droite => {
                            self.x = LARGEUR_ECRAN / 2;
                            self.y -= self.vitesse;
                        }
                        Direction::ToutDroit => {
                            self.x -= self.vitesse;
                        }
                    }
                }
            }
            Cote::Sud => {
                if self.y - self.vitesse > HAUTEUR_ECRAN / 2 + 2 * HAUTEUR_VOITURE {
                    self.y -= self.vitesse;
                } else if self.y - self.vitesse > HAUTEUR_ECRAN / 2 {
                    if feu.couleur == Feu::Vert {
                        self.y -= self.vitesse;
                    } else {
                        self.y = HAUTEUR_ECRAN / 2 + HAUTEUR_VOITURE;
                    }
                } else {
                    match self.direction {
                        Direction::Gauche => {
                            self.y = HAUTEUR_ECRAN / 2 - HAUTEUR_VOITURE;
                            self.x -= self.vitesse;
                        }
                        Direction::Droite => {
                            self.y = HAUTEUR_ECRAN / 2;
                            self.x += self.vitesse;
                        }
                        Direction::ToutDroit => {
                            self.y -= self.vitesse;
                        }
                    }
                }
            }
        }
    }
}

//Structure Route et son implémentation
#[derive(Debug, Clone, PartialEq, Eq)]
struct Route {
    pub voitures_avant_feu_nord: Vec<Voiture>,
    pub voitures_avant_feu_sud: Vec<Voiture>,
    pub voitures_avant_feu_est: Vec<Voiture>,
    pub voitures_avant_feu_ouest: Vec<Voiture>,
    pub voitures_dans_intersection: Vec<Voiture>,
    pub voitures_apres_feu_nord: Vec<Voiture>,
    pub voitures_apres_feu_sud: Vec<Voiture>,
    pub voitures_apres_feu_est: Vec<Voiture>,
    pub voitures_apres_feu_ouest: Vec<Voiture>,
    pub feu_nord: FeuTricolore,
    pub feu_est: FeuTricolore,
    pub feu_sud: FeuTricolore,
    pub feu_ouest: FeuTricolore,
}

impl Route {
    pub fn nouvelle() -> Route {
        Route {
            voitures_avant_feu_nord: vec![],
            voitures_avant_feu_sud: vec![],
            voitures_avant_feu_est: vec![],
            voitures_avant_feu_ouest: vec![],
            voitures_dans_intersection: vec![],
            voitures_apres_feu_nord: vec![],
            voitures_apres_feu_sud: vec![],
            voitures_apres_feu_est: vec![],
            voitures_apres_feu_ouest: vec![],
            feu_nord: FeuTricolore { couleur: Feu::Rouge },
            feu_est: FeuTricolore { couleur: Feu::Rouge },
            feu_sud: FeuTricolore { couleur: Feu::Rouge },
            feu_ouest: FeuTricolore { couleur: Feu::Rouge },
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Simulation Intersection Routière", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Créer les véhicules initiaux
    let mut route = Route::nouvelle();

    //Boucle principale de simulation
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                //Générer un véhicule aléatoire lors de l'appui sur une touche de direction
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    route.voitures_avant_feu_est
                        .push(Voiture::nouvelle(Cote::Est));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    route.voitures_avant_feu_sud
                        .push(Voiture::nouvelle(Cote::Sud));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    route.voitures_avant_feu_ouest
                        .push(Voiture::nouvelle(Cote::Ouest));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    route.voitures_avant_feu_nord
                        .push(Voiture::nouvelle(Cote::Nord));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let voiture_aleatoire = Voiture::voiture_aleatoire();
                    match voiture_aleatoire.cote {
                        Cote::Est => route.voitures_avant_feu_est.push(voiture_aleatoire),
                        Cote::Nord => route.voitures_avant_feu_nord.push(voiture_aleatoire),
                        Cote::Sud => route.voitures_avant_feu_sud.push(voiture_aleatoire),
                        Cote::Ouest => route.voitures_avant_feu_ouest.push(voiture_aleatoire),
                    }
                }
                _ => {}
            }
        }

        //Effacer le canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //Dessiner les voitures
        for voiture in &route.voitures_apres_feu_est {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_apres_feu_ouest {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_apres_feu_nord {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_apres_feu_sud {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_dans_intersection {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_avant_feu_est {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_avant_feu_ouest {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_avant_feu_nord {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }
        for voiture in &route.voitures_avant_feu_sud {
            canvas.set_draw_color(voiture.couleur);
            canvas
                .fill_rect(Rect::new(
                    voiture.x as i32,
                    voiture.y as i32,
                    LARGEUR_VOITURE as u32,
                    HAUTEUR_VOITURE as u32,
                ))
                .unwrap();
        }

        //Dessiner les routes
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_rect(Rect::new(
                380,
                0,
                LARGEUR_VOITURE as u32,
                HAUTEUR_ECRAN as u32,
            ))
            .unwrap();
        canvas
            .draw_rect(Rect::new(
                400,
                0,
                LARGEUR_VOITURE as u32,
                HAUTEUR_ECRAN as u32,
            ))
            .unwrap();
        canvas
            .draw_rect(Rect::new(
                0,
                380,
                LARGEUR_ECRAN as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();
        canvas
            .draw_rect(Rect::new(
                0,
                400,
                LARGEUR_ECRAN as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();

        //Dessiner les feux tricolores
        if route.feu_nord.couleur == Feu::Vert {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                LARGEUR_ECRAN / 2 - 2 * LARGEUR_VOITURE,
                HAUTEUR_ECRAN / 2 - 2 * HAUTEUR_VOITURE,
                LARGEUR_VOITURE as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();

        if route.feu_est.couleur == Feu::Vert {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                LARGEUR_ECRAN / 2 - 2 * LARGEUR_VOITURE,
                HAUTEUR_ECRAN / 2 + HAUTEUR_VOITURE,
                LARGEUR_VOITURE as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();

        if route.feu_sud.couleur == Feu::Vert {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                LARGEUR_ECRAN / 2 + LARGEUR_VOITURE,
                HAUTEUR_ECRAN / 2 + HAUTEUR_VOITURE,
                LARGEUR_VOITURE as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();

        if route.feu_ouest.couleur == Feu::Vert {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                LARGEUR_ECRAN / 2 + LARGEUR_VOITURE,
                HAUTEUR_ECRAN / 2 - 2 * HAUTEUR_VOITURE,
                LARGEUR_VOITURE as u32,
                HAUTEUR_VOITURE as u32,
            ))
            .unwrap();
/////main boucle tzad dyl simulation
        //Mettre à jour l'écran
        canvas.present();

        //Ajouter un délai pour contrôler les FPS
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}