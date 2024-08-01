import ImageDropzone from "../components/ImageDropzone";
import r2wc from "@r2wc/react-to-web-component";

function ImageDropzoneWrapped({
  "props-fn": propsFn,
}: {
  "props-fn": () => React.ComponentProps<typeof ImageDropzone>;
}) {
  return <ImageDropzone {...propsFn()} />;
}

const WebImageDropzone = r2wc(ImageDropzoneWrapped, {
  props: {
    "props-fn": "function",
  },
});

export const defineImageDropzone = () =>
  customElements.define("x-image-dropzone", WebImageDropzone);
