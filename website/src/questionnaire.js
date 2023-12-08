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
    resetAll();
    // make a get request at nuitinfo.feur.live/question
    const response = await fetch("http://nuitinfo.feur.live:8000/question");
    const data = await response.json();

    // create the questionnaire
    const question = data.question;
    const options = data.options;
    const answers = data.answer;
    const explanation = data.explanation;

    // put information
    const questionElement = document.getElementById("question");
    questionElement.textContent = question;

    const response1Element = document.getElementById("response1");
    response1Element.textContent = options[0];

    const response2Element = document.getElementById("response2");
    response2Element.textContent = options[1];

    const response3Element = document.getElementById("response3");
    response3Element.textContent = options[2];

    const response4Element = document.getElementById("response4");
    response4Element.textContent = options[3];

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

    const explanationElement = document.getElementById("explanation");
    explanationElement.textContent = "";
}

function validAnswer(answer) {
    resetAll();
    const explanationElement = document.getElementById("explanation");
    explanationElement.textContent = currentExplanation;


    if (answer === currentAnswer) {
        // good answer
        const responseElement = document.getElementById("response" + (answer + 1));
        responseElement.style.backgroundColor = "#2D6A4F";
        responseElement.style.color = "#FFFFFF";
    } else {
        // bad answer
        const badResponseElement = document.getElementById("response" + (answer + 1));
        badResponseElement.style.backgroundColor = "#FF0000";
        badResponseElement.style.color = "#FFFFFF";

        console.log(currentAnswer);
        const goodResponseElement = document.getElementById("response" + (currentAnswer + 1));
        goodResponseElement.style.backgroundColor = "#2D6A4F";
        goodResponseElement.style.color = "#FFFFFF";
    }
}