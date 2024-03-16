export default function myInitializer() {
  let startTime;

  return {
    onStart: () => {
      console.debug("Loading...");
      startTime = performance.now();
      document.getElementById("loading-container").style.display = "block";
      document.getElementById("loading-message").textContent = "Loading...";
    },
    onProgress: ({ current, total }) => {
      const progressBar = document.getElementById("loading-progress");
      const loadingMessage = document.getElementById("loading-message");

      if (total) {
        const percentage = Math.round((current / total) * 100);
        progressBar.style.width = `${percentage}%`;
        loadingMessage.textContent = `Loading... ${percentage}%`;
      } else {
        loadingMessage.textContent = "Loading...";
      }
    },
    onComplete: () => {
      const currentTime = performance.now();
      const elapsedTime = currentTime - startTime;
      const remainingTime = Math.max(100 - elapsedTime, 0);

      setTimeout(() => {
        console.debug("Loading... done!");
        document.getElementById("loading-container").style.display = "none";
      }, remainingTime);
    },
    onSuccess: (wasm) => {
      console.debug("Loading... successful!");
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
  };
}
