export function setFullscreenFunctions() {
  window.openFullScreen = function(id, calledDotnetObject) {
    openFullScreen(id, calledDotnetObject);
  }

  window.exitFullScreen = function () {
    exitFullScreen();
  }
}

let currentFullScreenElementId = null;
let currentDotnetReference = null;

function openFullScreen(id, calledDotnetObject) {
  let element = document.getElementById(id);
  currentFullScreenElementId = id;
  currentDotnetReference = calledDotnetObject;

  if (element.requestFullscreen) {
    element.requestFullscreen();
  } else if (element.mozRequestFullScreen) { /* Firefox */
    element.mozRequestFullScreen();
  } else if (element.webkitRequestFullscreen) { /* Chrome, Safari and Opera */
    element.webkitRequestFullscreen();
  } else if (element.msRequestFullscreen) { /* IE/Edge */
    element.msRequestFullscreen();
  }
}

function exitFullScreen() {
  currentDotnetReference = null;
  currentFullScreenElementId = null;

  if (document.exitFullscreen) {
    document.exitFullscreen();
  } else if (document.mozCancelFullScreen) { /* Firefox */
    document.mozCancelFullScreen();
  } else if (document.webkitExitFullscreen) { /* Chrome, Safari and Opera */
    document.webkitExitFullscreen();
  } else if (document.msExitFullscreen) { /* IE/Edge */
    document.msExitFullscreen();
  }
}

addEventListener("fullscreenchange", async (event) => {
  if (currentFullScreenElementId !== null && currentFullScreenElementId === event.target.id && currentDotnetReference !== null && !document.fullscreenElement) {
    await currentDotnetReference.invokeMethodAsync("HandleFullScreenExit");
    currentDotnetReference = null;
    currentFullScreenElementId = null;
  }
});
