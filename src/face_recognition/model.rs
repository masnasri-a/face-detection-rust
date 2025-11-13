use anyhow::Result;
use opencv::{
    core::{Mat, Vector, Size, Rect},
    face::LBPHFaceRecognizer,
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::{resize, INTER_LINEAR},
    objdetect::CascadeClassifier,
    prelude::*,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const FACE_SIZE: (i32, i32) = (200, 200);

pub struct FaceRecognitionModel {
    recognizer: opencv::core::Ptr<LBPHFaceRecognizer>,
    labels_map: HashMap<i32, String>, // label -> user_id
    cascade: CascadeClassifier,
    is_trained: bool,
}

impl FaceRecognitionModel {
    pub fn new() -> Result<Self> {
        let recognizer = LBPHFaceRecognizer::create(1, 8, 8, 8, 123.0)?;
        
        // Try multiple possible locations for haarcascade file
        let possible_paths = vec![
            "/usr/local/share/opencv4/haarcascades/haarcascade_frontalface_default.xml",
            "/opt/homebrew/share/opencv4/haarcascades/haarcascade_frontalface_default.xml",
            "/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml",
            "./haarcascade_frontalface_default.xml", // Local fallback
        ];
        
        let mut cascade_path = None;
        for path in possible_paths {
            if std::path::Path::new(path).exists() {
                cascade_path = Some(path);
                println!("Found haarcascade at: {}", path);
                break;
            }
        }
        
        let cascade_path = cascade_path.ok_or_else(|| {
            anyhow::anyhow!(
                "Haarcascade file not found. Please:\n\
                1. Install OpenCV: brew install opencv\n\
                2. Or download manually from:\n\
                   https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_default.xml\n\
                   Save to: ./haarcascade_frontalface_default.xml"
            )
        })?;
        
        let cascade = CascadeClassifier::new(cascade_path)?;
        
        // Verify cascade loaded successfully
        if cascade.empty()? {
            return Err(anyhow::anyhow!(
                "Failed to load cascade classifier from: {}", 
                cascade_path
            ));
        }
        
        Ok(Self {
            recognizer,
            labels_map: HashMap::new(),
            cascade,
            is_trained: false,
        })
    }

    pub fn train(&mut self, knowledge_path: &str) -> Result<()> {
        let mut images = Vector::<Mat>::new();
        let mut labels = Vector::<i32>::new();
        let mut current_label = 0;
        self.labels_map.clear();

        let knowledge_dir = Path::new(knowledge_path);
        if !knowledge_dir.exists() {
            fs::create_dir_all(knowledge_dir)?;
            return Ok(()); // No data to train yet
        }

        // Iterate through user directories
        for entry in fs::read_dir(knowledge_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let user_id = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                
                if user_id.is_empty() {
                    continue;
                }

                // Process all images in this user's directory
                for img_entry in fs::read_dir(&path)? {
                    let img_entry = img_entry?;
                    let img_path = img_entry.path();
                    
                    if img_path.is_file() {
                        if let Some(ext) = img_path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            if ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" {
                                match self.process_face_image(&img_path.to_string_lossy()) {
                                    Ok(face_mat) => {
                                        images.push(face_mat);
                                        labels.push(current_label);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to process {}: {}", img_path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }

                if !images.is_empty() {
                    self.labels_map.insert(current_label, user_id);
                    current_label += 1;
                }
            }
        }

        if !images.is_empty() {
            self.recognizer.train(&images, &labels)?;
            self.is_trained = true;
            println!("Model trained with {} images and {} users", images.len(), self.labels_map.len());
        }

        Ok(())
    }

    fn process_face_image(&mut self, image_path: &str) -> Result<Mat> {
        let img = imread(image_path, IMREAD_GRAYSCALE)?;
        
        if img.empty() {
            return Err(anyhow::anyhow!("Failed to load image"));
        }

        // Detect face in the image
        let mut faces = Vector::<Rect>::new();
        self.cascade.detect_multi_scale(
            &img,
            &mut faces,
            1.1,
            3,
            0,
            Size::new(30, 30),
            Size::new(0, 0),
        )?;

        if faces.is_empty() {
            return Err(anyhow::anyhow!("No face detected in image"));
        }

        // Use the first detected face
        let face_rect = faces.get(0)?;
        let face_roi = Mat::roi(&img, face_rect)?;
        
        // Resize to standard size
        let mut resized = Mat::default();
        resize(
            &face_roi,
            &mut resized,
            Size::new(FACE_SIZE.0, FACE_SIZE.1),
            0.0,
            0.0,
            INTER_LINEAR,
        )?;

        Ok(resized)
    }

    pub fn predict(&mut self, image_path: &str) -> Result<Option<String>> {
        if !self.is_trained {
            return Err(anyhow::anyhow!("Model not trained yet"));
        }

        let face_mat = self.process_face_image(image_path)?;
        
        let mut label = 0;
        let mut confidence = 0.0;
        self.recognizer.predict(&face_mat, &mut label, &mut confidence)?;

        println!("Predicted label: {}, confidence: {}", label, confidence);

        // Lower confidence means better match in LBPH
        // Typical threshold is around 50-80
        if confidence < 80.0 {
            if let Some(user_id) = self.labels_map.get(&label) {
                return Ok(Some(user_id.clone()));
            }
        }

        Ok(None)
    }

    // pub fn is_trained(&self) -> bool {
    //     self.is_trained
    // }
}
