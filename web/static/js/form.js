var form = $("#signup-form");
var url = "https://script.google.com/macros/s/AKfycbwcNQPjvYJntyjY1KsGK62rP1qe75yKA9iH_T9jC8dz6Ho_bR8/exec";

var successMessage = "You're signed up. We should be sending you an email shortly.";
var errorMessages = {
    "badValue": "One of the fields contains invalid input. Please check the form again and make sure that all required fields are filled out.",
    "alreadySignedUp": "You're already signed up. Please email us at <a href=\"mailto:churchillstemclub@gmail.com\">churchillstemclub@gmail.com</a> if you want to change something that you already submitted.",
    "noEmailQuota": "You have been registered as a member, but for technical reasons, we won't be able to email you automatically.",
    "sendEmail": "You have been registered as a member, but there was an error emailing you automatically."
}
var genericErrorMessage = "There was an error. Please email us at <a href=\"mailto:churchillstemclub@gmail.com\">churchillstemclub@gmail.com</a>.";

form.on("submit", function(e) {
    e.preventDefault();
    $.getJSON(url, form.serialize(), function(result) {
        var type = result["result"];
        if (type && (type === "error")) {
            var error = result["error"];
            if (error) {
                // For debugging. If you're reading this code after getting an error that you can't fix, email us the
                // output of this line so we can take a look into it.
                console.error(error);
                var errorType = error["type"];
                var customMessage = errorMessages[errorType];
                var message = customMessage ? customMessage : genericErrorMessage;
                changeSignUpBox(message);
            }
        } else if (type && type === "success") {
            changeSignUpBox(successMessage);
        }
    });
});

function changeSignUpBox(message) {
    var box = document.getElementById("results");

    // Clear box
    while (box.hasChildNodes()) {
        box.removeChild(box.lastChild);
    }

    var container = document.createElement("div");
    container.classList.add("top");

    var note = document.createElement("p");
    var noteText = document.createTextNode("");
    note.appendChild(noteText);

    container.innerHTML = message;

    container.appendChild(note);
    box.appendChild(container);
}