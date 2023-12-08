var currentAnswer = -1;
var currentExplanation = "";

export function initQuestionnaire() {
    const response1Element = document.getElementById("response1");
    response1Element.addEventListener("click", () => validAnswer(0));

    const response2Element = document.getElementById("response2");
    response2Element.addEventListener("click", () => validAnswer(1));

    const response3Element = document.getElementById("response3");
    response3Element.addEventListener("click", () => validAnswer(2));

    const response4Element = document.getElementById("response4");
    response4Element.addEventListener("click", () => validAnswer(3));

    const nextQuestionElement = document.getElementById("next-question");
    nextQuestionElement.addEventListener("click", () => createQuestionnaire());

    createQuestionnaire();
}

async function createQuestionnaire() {
    // make a get request at nuitinfo.feur.live/question
    const response = await fetch("https://nuitinfo.feur.live/question");
    const data = await response.json();

    // create the questionnaire
    const question = data.question;
    const options = data.options;
    const answers = data.answers;
    const explanation = data.explanation;

    // put information
    const questionElement = document.getElementById("question");
    questionElement.innerHTML = question;

    const response1Element = document.getElementById("response1");
    response1Element.innerHTML = options[0];

    const response2Element = document.getElementById("response2");
    response2Element.innerHTML = options[1];

    const response3Element = document.getElementById("response3");
    response3Element.innerHTML = options[2];

    const response4Element = document.getElementById("response4");
    response4Element.innerHTML = options[3];

    currentAnswer = answers;
    currentExplanation = explanation;
}

function resetAll() {
    const response1Element = document.getElementById("response1");
    response1Element.style.backgroundColor = "#FFFFFF";
    response1Element.style.color = "#000000";

    const response2Element = document.getElementById("response2");
    response2Element.style.backgroundColor = "#FFFFFF";
    response2Element.style.color = "#000000";

    const response3Element = document.getElementById("response3");
    response3Element.style.backgroundColor = "#FFFFFF";
    response3Element.style.color = "#000000";

    const response4Element = document.getElementById("response4");
    response4Element.style.backgroundColor = "#FFFFFF";
    response4Element.style.color = "#000000";
}

function validAnswer(answer) {
    resetAll();
    const explanationElement = document.getElementById("explanation");
    explanationElement.innerHTML = currentExplanation;


    if (answer === currentAnswer) {
        // good answer
        const responseElement = document.getElementById("response" + (answer + 1));
        responseElement.style.backgroundColor = "#52B788";
        responseElement.style.color = "#FFFFFF";
    } else {
        // bad answer
        const badResponseElement = document.getElementById("response" + (answer + 1));
        badResponseElement.style.backgroundColor = "#FF0000";
        badResponseElement.style.color = "#FFFFFF";

        const goodResponseElement = document.getElementById("response" + (currentAnswer + 1));
        goodResponseElement.style.backgroundColor = "#52B788";
        goodResponseElement.style.color = "#FFFFFF";
    }
}