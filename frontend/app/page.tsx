import Link from 'next/link';

export default function Home() {
  return (
    <div className="min-h-screen bg-linear-to-br from-blue-50 to-purple-100 p-8">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-5xl font-bold text-gray-800 mb-4">
            🎭 Face Recognition System
          </h1>
          <p className="text-xl text-gray-600">
            Real-time face detection and recognition powered by OpenCV
          </p>
        </div>

        <div className="grid md:grid-cols-2 gap-8 mb-12">
          <Link href="/add-face" className="group">
            <div className="bg-white rounded-lg shadow-xl p-8 hover:shadow-2xl transition-all transform hover:scale-105">
              <div className="text-6xl mb-4">📸</div>
              <h2 className="text-2xl font-bold text-gray-800 mb-3">Add Face</h2>
              <p className="text-gray-600 mb-4">
                Capture multiple photos and train the face recognition model with a new user.
              </p>
              <ul className="text-sm text-gray-500 space-y-2">
                <li>✅ Camera access</li>
                <li>✅ Multiple photo capture</li>
                <li>✅ Auto training</li>
              </ul>
              <div className="mt-4 text-blue-600 font-semibold group-hover:translate-x-2 transition-transform inline-block">
                Get Started →
              </div>
            </div>
          </Link>

          <Link href="/detect" className="group">
            <div className="bg-white rounded-lg shadow-xl p-8 hover:shadow-2xl transition-all transform hover:scale-105">
              <div className="text-6xl mb-4">🔍</div>
              <h2 className="text-2xl font-bold text-gray-800 mb-3">Detect Face</h2>
              <p className="text-gray-600 mb-4">
                Real-time face detection and recognition with automatic identification.
              </p>
              <ul className="text-sm text-gray-500 space-y-2">
                <li>✅ Real-time detection</li>
                <li>✅ Auto recognition</li>
                <li>✅ User identification</li>
              </ul>
              <div className="mt-4 text-purple-600 font-semibold group-hover:translate-x-2 transition-transform inline-block">
                Start Detection →
              </div>
            </div>
          </Link>
        </div>

        <div className="bg-white rounded-lg shadow-xl p-8">
          <h3 className="text-2xl font-bold text-gray-800 mb-4">How It Works</h3>
          <div className="space-y-4">
            <div className="flex gap-4">
              <div className="flex-shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold">
                1
              </div>
              <div>
                <h4 className="font-semibold text-gray-800">Add Training Data</h4>
                <p className="text-gray-600">Go to "Add Face" page and capture 5-10 photos from different angles</p>
              </div>
            </div>
            <div className="flex gap-4">
              <div className="flex-shrink-0 w-8 h-8 bg-purple-600 text-white rounded-full flex items-center justify-center font-bold">
                2
              </div>
              <div>
                <h4 className="font-semibold text-gray-800">Auto Training</h4>
                <p className="text-gray-600">The system automatically trains the model using LBPH algorithm</p>
              </div>
            </div>
            <div className="flex gap-4">
              <div className="flex-shrink-0 w-8 h-8 bg-green-600 text-white rounded-full flex items-center justify-center font-bold">
                3
              </div>
              <div>
                <h4 className="font-semibold text-gray-800">Real-time Detection</h4>
                <p className="text-gray-600">Use "Detect Face" to identify faces in real-time</p>
              </div>
            </div>
          </div>
        </div>

        <div className="mt-8 text-center text-gray-600">
          <p className="text-sm">
            Powered by OpenCV • LBPH Algorithm • SQLite • Next.js 16
          </p>
        </div>
      </div>
    </div>
  );
}
