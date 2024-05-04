export default function myInitializer() {
  let startTime = 0;

  return {
    onStart: () => {
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
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
  };
}
