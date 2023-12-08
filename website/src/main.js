import '../style/style.css';
import '../style/counter.css';
import Rellax from 'rellax';
import AOS from 'aos';
import * as THREE from 'three';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js';

import { startCooldownClock } from "./finalCountdown.js";
import {initScrollManager} from "./scrollManager.js";
import {initQuestionnaire} from "./questionnaire.js";


const objUrl = 'model/low-poly-mill.obj';
const mtlUrl = 'model/low-poly-mill.mtl';

let rellax = new Rellax('.rellax');
AOS.init();


// Threejs
document.addEventListener('DOMContentLoaded', async () => {
    const heroModelContainer = document.getElementById('hero-model');

    const heroModelRect = heroModelContainer.getBoundingClientRect();
    const canvasWidth = heroModelRect.width;
    const canvasHeight = heroModelRect.height;

    const canvas = document.createElement('canvas');
    canvas.width = canvasWidth;
    canvas.height = canvasWidth;

    heroModelContainer.appendChild(canvas);

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, canvas.width / canvas.height, 0.1, 1000);
    camera.position.z = 20;
    camera.position.y = 10;
    camera.rotateX(-0.3);

    const renderer = new THREE.WebGLRenderer({ canvas, alpha: true });
    renderer.setSize(canvas.width, canvas.height);
    renderer.setClearColor(0x000000, 0);

    // Loaders
    const objLoader = new OBJLoader();
    const mtlLoader = new MTLLoader();

    const pointLight = new THREE.PointLight(0xdddddddd, 2000, 100000); // color, intensity, distance
    pointLight.position.set(0, 20, 30);
    scene.add(pointLight);

    // Load materials first
    mtlLoader.load(mtlUrl, (materials) => {
        materials.preload();
        objLoader.setMaterials(materials);

        // Then load object
        objLoader.load(objUrl, (object) => {
            object.rotateY(1.1)
            object.position.y = -5;
            object.scale.set(0.2, 0.2, 0.2); // Scale to half the original size

            scene.add(object);
            render();
        });
    });

    function render() {
        requestAnimationFrame(render);
        renderer.render(scene, camera);
    }

    startCooldownClock();
    initScrollManager();
    initQuestionnaire();
});

