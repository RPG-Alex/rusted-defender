<a name="readme-top"></a>

# Rusted Defender

<div align="center"> 
    <img src="images/logo.png" alt="Logo" width="80" height="80"> 
</div>

<details> 
    <summary>Contents</summary>
     <ol> 
        <li> 
            <a href="#introduction">Introduction</a> 
        </li> 
        <li> 
            <a href="#roadmap">Roadmap</a> 
        </li> 
        <li> 
            <a href="#stay-tuned">Stay Tuned!</a> 
        </li> 
        <li> 
            <a href="#development-and-running-locally">Development and Running locally</a> 
            </li> 
    </ol> 
</details>

## Introduction

Welcome to Rusted Defender, a thrilling and action-packed game promising excitement and adventure! Prepare to embark on an unforgettable journey through a world of rust and adventure. Rusted Defender offers an exciting gaming experience, designed especially for primary school learners. Enter a world teeming with challenges, adversaries, and incredible adventures while enjoying a fantastic time.
Status

Please note that this project is currently under construction. Please check back for updates!

## Roadmap

### Done Stuff:

- [x] Player sprite control implemented.
- [x] Collision mechanics incorporated.
- [x] Projectile and firing functionality realized.
- [x] Refactor code using the new sprite attributes struct.

### Not Done:


- [ ] Develop a splash screen.
    - [ ] Design a visually appealing splash screen.
    - [ ] Implement a loading animation.
- [ ] Create a start menu screen.
    - [ ] Design the layout of the start menu.
    - [ ] Add buttons for ‘Start Game’, ‘Settings’, ‘High Scores’, etc.
- [ ] Implement a scoring/lives system.
    - [ ] Design a scoring algorithm.
    - [ ] Display the score and lives on the game screen.
    - [ ] Implement game over logic when lives run out.
- [ ] Establish a comprehensive roadmap for the game overall.
    - [ ] Define the game’s core mechanics and features.
    - [ ] Plan the development phases and timelines.
- [ ] Implement sound effects and background music.
    - [ ] Choose or create suitable sound effects for game actions.
    - [ ] Add background music to enhance the gaming experience.
- [ ] Create multiple levels of difficulty.
    - [ ] Design levels with increasing difficulty.
    - [ ] Implement level progression logic.
- [ ] Add power-ups and bonuses.
    - [ ] Design various power-ups and bonuses.
    - [ ] Implement the logic for spawning and using power-ups.
- [ ] Implement a save/load game feature.
    - [ ] Design a system for saving and loading game progress.
    - [ ] Implement the save/load feature in the game menu.
- [ ] Optimize the game for performance.
    - [ ] Profile the game to identify performance bottlenecks.
    - [ ] Optimize the game code and assets for smooth gameplay.
- [ ] Test the game thoroughly.
    - [ ] Perform unit testing for individual game components.
    - [ ] Conduct playtesting to gather feedback on game mechanics and difficulty.
- [ ] Prepare the game for release.
    - [ ] Package the game for distribution.
    - [ ] Create promotional materials for the game release.
- [ ] More to come? Absolutely, game development is an iterative and creative process! Keep the ideas flowing.


## Stay Tuned!

Excitement abounds as we prepare to share Rusted Defender with you, and updates will be provided on our progress. Be sure to check back for the latest news and announcements as we work to create a memorable experience for players of all ages. Your interest in Rusted Defender is greatly appreciated!
Development and Running locally:

## Development and Run Locally

If you are interested in developing and working on the game thats great! The game is in a pretty early state of development so please do feel free to start contributing!

### Run locally 

#### Install Rust:
        First, you need to install Rust on your system. You can download it from the official Rust website.
        Follow the instructions on the website to install Rust. This will also install cargo, Rust’s package manager.

#### Clone the Repository:
        Clone the game repository to your local machine using Git. If you don’t have Git installed, you can download it from the official Git website.
        Open a terminal/command prompt, navigate to the directory where you want to clone the repository, and run the following command:

    git clone <repository_url>

    Replace <repository_url> with the URL of the game’s Git repository.

#### Build and Run the Game:
        Navigate into the cloned repository’s directory:

    cd <repository_name>

    Replace <repository_name> with the name of the game’s repository.
        Build and run the game using cargo with the following command:

    cargo run --features bevy/dynamic_linking

    This command tells cargo to run the game, and the --features bevy/dynamic_linking part is a feature flag for the Bevy game engine used in this project.

And that’s it! The game should now be running on your local machine. Enjoy playing!

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
