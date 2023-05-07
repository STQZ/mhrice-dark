let darkMode = localStorage.getItem("darkMode");
const darkModeToggle = document.querySelector("#dark-mode-toggle")


const enableDarkMode = () => {
    document.getElementById("stylething").setAttribute('href', "/mhrice-dark.css?h={}");
    localStorage.setItem("darkMode", "enabled");
};

const disableDarkMode = () => {
    document.getElementById("stylething").setAttribute('href', "/mhrice.css?h={}");
    localStorage.setItem("darkMode", null);
};

if (darkMode === "enabled") {
    enableDarkMode();
}

darkModeToggle.addEventListener("click", () => {
    darkMode = localStorage.getItem("darkMode");
    if (darkMode !== "enabled") {
        enableDarkMode();
    } else {
        disableDarkMode();
    }
});
