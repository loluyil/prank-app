const { invoke } = window.__TAURI__.core;

function main() {
  const random = Math.floor(Math.random() * 3);
  const minutes = [1, 3, 5];
  const delay = minutes[random] * 60 * 1000;

  setTimeout(() => {
    randomSystem();
    main();
  }, delay);

  console.log("Random number was:", random);
  console.log(`Next execution in: ${minutes[random]} minutes`);
}

//Gacha system XD
function randomSystem() {
  const random = Math.floor(Math.random() * 100);
  
  if (random >= 0 && random < 40) {
    audioFunction();
    console.log("Audio activated");
    console.log("Common: ", random);
  } else if (random >= 40 && random < 70) {
    cursorFunction();
    console.log("Cursor activated");
    console.log("Epic: ", random);
  } else if (random >= 70) { 
    popupFunction();
    console.log("Popup activated");
    console.log("Legendary: ", random);
  }
}

function popupFunction() {
  const random = Math.random();

  if(random < 0.5) {
    fadePopup();
  } else {
    popupJumpscare();
  }
}

function popupJumpscare() {
  try {
    let time = 700;
    const audio = new Audio('foxyscream.mp3');
    audio.play();
    setTimeout(() => {
      invoke('toggle_popup_window');
    }, time);
  } catch (error) {
      console.error('Failed to toggle window:', error);
  }
}

function fadePopup() {
  try {
    const audio = new Audio('amongus.mp3');
    audio.volume = 0.25;
    
    //fixes audio sync on production
    audio.addEventListener('playing', () => {
      invoke('fade_popup_window');
    });
    
    audio.play();
  } catch (error) {
    console.error('Failed to toggle window:', error);
  }
}

function audioFunction() {
  const random = Math.floor(Math.random() * 3);
  let audio;

  switch(random) {
    case 0:
      audio = new Audio('discord-notif.mp3');
      console.log("Number:" + random);
      break;
    case 1:
      audio = new Audio('knock.mp3');
      console.log("Number:" + random);
      break;
    case 2:
      audio = new Audio('iphone-notification.mp3');
      console.log("Number:" + random);
      break;
  }

  if (audio) {
    audio.volume = 0.5;
    audio.play();
    console.log("Number:" + random);
    console.log("Volume =" + audio.volume);
  }
}

function cursorFunction() {
  const random = Math.random();

  if(random < 0.5) {
    cursorMove();
  } else {
    cursorCorner();
  }
}

async function cursorCorner() {
  try {
    for(let i = 0; i <= 7; i++) {
      await invoke('cursor_corners');
      console.log("Cursor moved to corners");
      }
    } catch (error) {
      console.error('Failed to move cursor:', error);
    }
}

async function cursorMove() {
  try {
    const [startingX, startingY] = await invoke('grab_mouse_position');
    let x = 0;
    let duration = 0.5;

    await invoke('cursor_move', {
      targetX: x,
      targetY: startingY,
      durationSecs: duration
    });
  } catch (error) {
    console.error('Failed to grab mouse position:', error);
  }
}

main();

//Test button

document.getElementById('test-button').addEventListener('click', () => {
  popupFunction();
});

document.getElementById('audio-button').addEventListener('click', () => {
  audioFunction();
});

document.getElementById('mouse-button').addEventListener('click', () => {
  cursorFunction();
})
