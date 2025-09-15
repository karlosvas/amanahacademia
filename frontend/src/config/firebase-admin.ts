import { initializeApp, applicationDefault, getApps, getApp } from "firebase-admin/app";
import { getAuth } from "firebase-admin/auth";

const app =
  getApps().length === 0
    ? initializeApp({
        credential: applicationDefault(),
        projectId: import.meta.env.PUBLIC_FIREBASE_PROJECT_ID,
      })
    : getApp();

export const adminAuth = getAuth(app);
