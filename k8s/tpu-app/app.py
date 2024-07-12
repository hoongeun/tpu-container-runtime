import numpy as np
from PIL import Image
import tflite_runtime.interpreter as tflite

# Load the TFLite model and allocate tensors
interpreter = tflite.Interpreter(model_path='/path/to/your/model.tflite', 
                                 experimental_delegates=[tflite.load_delegate('libedgetpu.so.1')])
interpreter.allocate_tensors()

# Get input and output tensors
input_details = interpreter.get_input_details()
output_details = interpreter.get_output_details()

# Load an image
image = Image.open('/path/to/your/image.jpg').convert('RGB')
input_shape = input_details[0]['shape']
image = image.resize((input_shape[2], input_shape[1]))

# Preprocess the image to the format the model expects
input_data = np.expand_dims(image, axis=0)
input_data = np.float32(input_data)

# Run inference
interpreter.set_tensor(input_details[0]['index'], input_data)
interpreter.invoke()

# The function `get_tensor()` returns a copy of the tensor data
# Use `tensor()` in cases where you want to avoid the copy
output_data = interpreter.get_tensor(output_details[0]['index'])

# Post-process the output to get the detected classes
scores = output_data[0][:, 4]
classes = output_data[0][:, 5].astype(np.int32)

# Output results
for score, cls in zip(scores, classes):
    print('Class ID: {}, Score: {}'.format(cls, score))
