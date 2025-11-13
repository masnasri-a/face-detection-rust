'use client';

import { useRef, useState, useEffect } from 'react';
import Webcam from 'react-webcam';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export default function DetectFacePage() {
  const webcamRef = useRef<Webcam>(null);
  const [isDetecting, setIsDetecting] = useState(false);
  const [detectionResult, setDetectionResult] = useState<{
    userId: string | null;
    detected: boolean;
    timestamp: string;
  } | null>(null);
  const [hasPermission, setHasPermission] = useState<boolean | null>(null);
  const [error, setError] = useState<string | null>(null);

  const detectFace = async () => {
    if (!webcamRef.current) return;

    const imageSrc = webcamRef.current.getScreenshot();
    if (!imageSrc) return;

    setIsDetecting(true);
    setError(null);

    try {
      const blob = await fetch(imageSrc).then(r => r.blob());
      const formData = new FormData();
      formData.append('photo', blob, 'capture.jpg');

      const response = await fetch(`${API_URL}/detect-face`, {
        method: 'POST',
        body: formData,
      });

      const data = await response.json();

      if (response.ok && data.success) {
        setDetectionResult({
          userId: data.data.user_id,
          detected: data.data.detected,
          timestamp: new Date().toLocaleTimeString(),
        });
      } else {
        setError(data.message || 'Detection failed');
      }
    } catch (err) {
      console.error('Detection error:', err);
      setError('Network error. Please make sure the backend is running.');
    } finally {
      setIsDetecting(false);
    }
  };

  // Auto-detect every 2 seconds when enabled
  useEffect(() => {
    if (!isDetecting) return;

    const interval = setInterval(() => {
      detectFace();
    }, 2000);

    return () => clearInterval(interval);
  }, [isDetecting]);

  const toggleDetection = () => {
    setIsDetecting(!isDetecting);
    if (!isDetecting) {
      setDetectionResult(null);
      setError(null);
    }
  };

  return (
    <div className="min-h-screen bg-linear-to-br from-purple-50 to-pink-100 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold text-gray-800 mb-2">Detect Face</h1>
        <p className="text-gray-600 mb-8">Real-time face detection and recognition</p>

        <div className="bg-white rounded-lg shadow-xl p-6">
          <div className="relative mb-6">
            {hasPermission === false && (
              <div className="absolute inset-0 flex items-center justify-center bg-gray-100 rounded-lg z-10">
                <div className="text-center p-6">
                  <p className="text-red-600 font-semibold mb-2">Camera Access Denied</p>
                  <p className="text-sm text-gray-600">
                    Please allow camera access to use this feature
                  </p>
                </div>
              </div>
            )}

            <Webcam
              ref={webcamRef}
              audio={false}
              screenshotFormat="image/jpeg"
              className="w-full rounded-lg"
              videoConstraints={{
                width: 1280,
                height: 720,
                facingMode: "user"
              }}
              onUserMedia={() => setHasPermission(true)}
              onUserMediaError={() => setHasPermission(false)}
            />

            {/* Face detection overlay */}
            <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 border-4 border-purple-500 rounded-full opacity-30 pointer-events-none"></div>

            {/* Detection status indicator */}
            {isDetecting && (
              <div className="absolute top-4 right-4 flex items-center gap-2 bg-green-500 text-white px-4 py-2 rounded-full shadow-lg">
                <div className="w-3 h-3 bg-white rounded-full animate-pulse"></div>
                <span className="font-semibold">Detecting...</span>
              </div>
            )}
          </div>

          <div className="flex gap-4 mb-6">
            <button
              onClick={toggleDetection}
              className={`flex-1 font-bold py-3 px-6 rounded-lg shadow-lg transition-all hover:scale-105 ${
                isDetecting
                  ? 'bg-red-600 hover:bg-red-700 text-white'
                  : 'bg-purple-600 hover:bg-purple-700 text-white'
              }`}
            >
              {isDetecting ? '⏸️ Stop Detection' : '▶️ Start Auto Detection'}
            </button>
            <button
              onClick={detectFace}
              disabled={isDetecting}
              className="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-lg shadow-lg transition-all hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              📸 Detect Once
            </button>
          </div>

          {detectionResult && (
            <div className={`p-6 rounded-lg border-2 ${
              detectionResult.detected
                ? 'bg-green-50 border-green-500'
                : 'bg-yellow-50 border-yellow-500'
            }`}>
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-lg font-bold text-gray-800">
                  {detectionResult.detected ? '✅ Face Recognized!' : '❌ Unknown Face'}
                </h3>
                <span className="text-sm text-gray-500">{detectionResult.timestamp}</span>
              </div>
              
              {detectionResult.detected && detectionResult.userId ? (
                <div>
                  <p className="text-sm text-gray-600 mb-2">Detected User:</p>
                  <p className="text-3xl font-bold text-green-700">{detectionResult.userId}</p>
                </div>
              ) : (
                <p className="text-gray-600">
                  No matching face found in the database. Please add this face first.
                </p>
              )}
            </div>
          )}

          {error && (
            <div className="p-4 rounded-lg bg-red-50 text-red-800 border border-red-200">
              ⚠️ {error}
            </div>
          )}

          <div className="mt-6 p-4 bg-blue-50 rounded-lg">
            <p className="text-sm text-blue-800">
              <strong>💡 How it works:</strong> Click "Start Auto Detection" for continuous real-time detection every 2 seconds, or use "Detect Once" for a single detection.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
