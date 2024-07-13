import argparse
import time
import numpy as np
from PIL import Image
from tflite_runtime.interpreter import Interpreter, load_delegate

def load_labels(filename):
    with open(filename, 'r') as f:
        return {i: line.strip() for i, line in enumerate(f.readlines())}

def main():
    model_path = './model/mobilenet_v2_1.0_224_inat_bird_quant_edgetpu.tflite'
    label_path = './model/inat_bird_labels.txt'

    parser = argparse.ArgumentParser(description='Edge TPU Image Classification')
    parser.add_argument('-i', '--input', required=True, help='Path to the input image')
    args = parser.parse_args()

    input_image_path = args.input

    # Load labels
    labels = load_labels(label_path)

    # Initialize the Edge TPU interpreter
    interpreter = Interpreter(
        model_path,
        experimental_delegates=[load_delegate('libedgetpu.so.1')]
    )
    interpreter.allocate_tensors()

    # Model must be uint8 quantized
    input_details = interpreter.get_input_details()
    output_details = interpreter.get_output_details()
    input_shape = input_details[0]['shape']

    if input_shape[1] != 224 or input_shape[2] != 224:
        raise ValueError('Only support model with input shape 224x224')

    # Load and preprocess the input image
    image = Image.open(input_image_path).convert('RGB').resize((224, 224), Image.LANCZOS)
    input_tensor = np.expand_dims(image, axis=0)

    if input_details[0]['dtype'] == np.float32:
        input_tensor = (np.float32(input_tensor) - 127.5) / 127.5
    else:
        input_tensor = np.uint8(input_tensor)

    # Run inference
    print('----INFERENCE TIME----')
    print('Note: The first inference on Edge TPU is slow because it includes',
          'loading the model into Edge TPU memory.')
    for i in range(5):  # Hardcoded number of inference runs
        start = time.perf_counter()
        interpreter.set_tensor(input_details[0]['index'], input_tensor)
        interpreter.invoke()
        inference_time = time.perf_counter() - start

        # Post-processing
        output = interpreter.get_tensor(output_details[0]['index'])[0]
        top_k = output.argsort()[-5:][::-1]  # Top 5 classes

        print('%.1fms' % (inference_time * 1000))

    # Print results
    print('-------RESULTS--------')
    for i in top_k:
        print('%s: %.5f' % (labels[i], output[i] / 255.0))

if __name__ == '__main__':
    main()
