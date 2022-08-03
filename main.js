let menuToggle = document.querySelector('.menu-toggle');

let text = document.querySelector(".menu-toggle > ion-icon")
let menuScreen = document.querySelector(".navbar-links")

menuToggle.addEventListener("click", () => {
    if (text.getAttribute("name") == "menu-outline") {
        // Open menu
        text.setAttribute("name", "close-outline")
        menuScreen.classList.add("open")
    } else {
        // Close menu
        text.setAttribute("name", "menu-outline")
        menuScreen.classList.remove("open")
    }
})

let menuItems = document.querySelectorAll(".navbar-links")
for (link of menuItems) {
    link.addEventListener("click", function () {
        text.setAttribute("name", "menu-outline")
        menuScreen.classList.remove("open")
        return true; // Not needed, as long as you don't return false
    });
}
