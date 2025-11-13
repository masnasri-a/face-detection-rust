'use client';

import { useState } from 'react';
import Camera from '../components/Camera';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export default function AddFacePage() {
  const [userId, setUserId] = useState('');
  const [capturedImages, setCapturedImages] = useState<string[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error' | 'info', text: string } | null>(null);

  const handleCapture = (imageSrc: string) => {
    setCapturedImages(prev => [...prev, imageSrc]);
    setMessage({ type: 'success', text: `Photo ${capturedImages.length + 1} captured!` });
  };

  const removeImage = (index: number) => {
    setCapturedImages(prev => prev.filter((_, i) => i !== index));
  };

  const handleSubmit = async () => {
    if (!userId.trim()) {
      setMessage({ type: 'error', text: 'Please enter a User ID' });
      return;
    }

    if (capturedImages.length === 0) {
      setMessage({ type: 'error', text: 'Please capture at least one photo' });
      return;
    }

    setIsSubmitting(true);
    setMessage({ type: 'info', text: 'Uploading photos and training model...' });

    try {
      const formData = new FormData();
      formData.append('id', userId);

      // Convert base64 images to blobs and append to FormData
      for (let i = 0; i < capturedImages.length; i++) {
        const base64 = capturedImages[i];
        const blob = await fetch(base64).then(r => r.blob());
        formData.append('photos', blob, `photo_${i + 1}.jpg`);
      }

      const response = await fetch(`${API_URL}/add-face`, {
        method: 'POST',
        body: formData,
      });

      const data = await response.json();

      if (response.ok && data.success) {
        setMessage({ 
          type: 'success', 
          text: `Successfully added ${data.data.images_saved} photos for user "${data.data.user_id}" and trained the model!` 
        });
        setCapturedImages([]);
        setUserId('');
      } else {
        setMessage({ type: 'error', text: data.message || 'Failed to add face data' });
      }
    } catch (error) {
      console.error('Error:', error);
      setMessage({ type: 'error', text: 'Network error. Please make sure the backend is running.' });
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold text-gray-800 mb-2">Add Face</h1>
        <p className="text-gray-600 mb-8">Capture multiple photos to train the face recognition model</p>

        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="mb-6">
            <label htmlFor="userId" className="block text-sm font-medium text-gray-700 mb-2">
              User ID <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="userId"
              value={userId}
              onChange={(e) => setUserId(e.target.value)}
              placeholder="Enter unique user ID (e.g., john_doe)"
              className="w-full px-4 py-2 border text-black border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          <div className="mb-6">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Camera Feed
            </label>
            <Camera onCapture={handleCapture} className="mb-4" />
            <p className="text-sm text-gray-500 text-center">
              💡 Tip: Capture 5-10 photos from different angles for better accuracy
            </p>
          </div>

          {capturedImages.length > 0 && (
            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Captured Photos ({capturedImages.length})
              </label>
              <div className="grid grid-cols-3 md:grid-cols-5 gap-4">
                {capturedImages.map((img, index) => (
                  <div key={index} className="relative group">
                    <img 
                      src={img} 
                      alt={`Captured ${index + 1}`} 
                      className="w-full h-32 object-cover rounded-lg shadow-md"
                    />
                    <button
                      onClick={() => removeImage(index)}
                      className="absolute top-1 right-1 bg-red-500 text-white rounded-full w-6 h-6 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity hover:bg-red-600"
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {message && (
            <div className={`p-4 rounded-lg mb-6 ${
              message.type === 'success' ? 'bg-green-50 text-green-800 border border-green-200' :
              message.type === 'error' ? 'bg-red-50 text-red-800 border border-red-200' :
              'bg-blue-50 text-blue-800 border border-blue-200'
            }`}>
              {message.text}
            </div>
          )}

          <div className="flex gap-4">
            <button
              onClick={handleSubmit}
              disabled={isSubmitting || !userId || capturedImages.length === 0}
              className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg shadow-lg transition-all hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              {isSubmitting ? '⏳ Processing...' : '✅ Submit & Train'}
            </button>
            <button
              onClick={() => {
                setCapturedImages([]);
                setUserId('');
                setMessage(null);
              }}
              className="px-6 py-3 border-2 border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              🔄 Reset
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
