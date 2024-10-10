// Import the functions you need from the SDKs you need

import { initializeApp } from "firebase/app";

import { getAnalytics } from "firebase/analytics";
import { getFirestore } from "firebase/firestore";
// TODO: Add SDKs for Firebase products that you want to use

// https://firebase.google.com/docs/web/setup#available-libraries


// Your web app's Firebase configuration

// For Firebase JS SDK v7.20.0 and later, measurementId is optional

const firebaseConfig = {

  apiKey: "AIzaSyCWBSqKl4sNvtnc7QrWdIsd7VfNPv8tnEQ",

  authDomain: "kingdom-system-demo.firebaseapp.com",

  projectId: "kingdom-system-demo",

  storageBucket: "kingdom-system-demo.appspot.com",

  messagingSenderId: "857628315888",

  appId: "1:857628315888:web:62592addfbb2542bfa3235",

  measurementId: "G-05Y33GN11W"

};


// Initialize Firebase

const app = initializeApp(firebaseConfig);
export const firestore = getFirestore(app);
