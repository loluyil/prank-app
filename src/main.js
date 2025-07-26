const { invoke } = window.__TAURI__.core;

function main() {
  const random = Math.floor(Math.random() * 3);
  const minutes = [0.5, 1, 2];
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
  const random = Math.floor(Math.random() * 2);

  switch(random) {
    case 2:
      popupJumpscare();
      console.log("Popup Jumpscare activated");
      break;
    case 1:
      fadePopup();
      console.log("Fade Popup activated");
      break;
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
    audio.play();
    invoke('fade_popup_window');
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
  const random = Math.floor(Math.random() * 2);

  switch(random) {
    case 0:
      cursorCorner();
      break;
    case 1:
      cursorFunction();
      break;
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


async function terminateProgram() {
    try {
        await invoke('terminate_program');
        console.log('Kill hotkey (Ctrl+Alt+P) registered');
    } catch (error) {
        console.error('Failed to setup hotkey:', error);
    }
}

//Kill switch
terminateProgram();

main();

//Test button
/*
document.getElementById('test-button').addEventListener('click', () => {
  popupFunction();
});

document.getElementById('audio-button').addEventListener('click', () => {
  audioFunction();
});

*/
