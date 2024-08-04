import { useMemo, useState } from "react";
import React, { useCallback } from "react";
import { useDropzone } from "react-dropzone";

const baseStyle: React.CSSProperties = {
  width: "100%",
  flex: 1,
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  padding: "20px",
  borderWidth: 2,
  borderRadius: 2,
  borderColor: "#eeeeee",
  borderStyle: "dashed",
  backgroundColor: "#fafafa",
  color: "#8d8d8d",
  outline: "none",
  transition: "border .24s ease-in-out",
  position: "relative",
};

const focusedStyle = {
  borderColor: "#2196f3",
};

const acceptStyle = {
  borderColor: "#00e676",
};

const rejectStyle = {
  borderColor: "#ff1744",
};

export type ImageDropzoneProps = Readonly<{
  onImageSelected?: (image: File) => void;
}>;
type OnDrop = NonNullable<
  NonNullable<Parameters<typeof useDropzone>[0]>["onDrop"]
>;
export default function ImageDropzone({ onImageSelected }: ImageDropzoneProps) {
  const [selectedImage, setSelectedImage] = useState<File>();
  const onDrop = useCallback<OnDrop>(
    (acceptedFiles) => {
      const file = acceptedFiles[0];
      if (file == null) return;
      setSelectedImage(file);
      onImageSelected?.(file);
    },
    [onImageSelected],
  );
  const selected = selectedImage != null;
  const {
    getRootProps,
    getInputProps,
    isFocused,
    isDragAccept,
    isDragReject,
    isDragActive,
    open,
  } = useDropzone({
    onDrop,
    noClick: selected,
    maxFiles: 1,
    accept: {
      "image/*": [],
    },
  });

  const style = useMemo<React.CSSProperties>(
    () => ({
      ...baseStyle,
      ...(isFocused ? focusedStyle : null),
      ...(isDragAccept ? acceptStyle : null),
      ...(isDragReject ? rejectStyle : null),
    }),
    [isFocused, isDragAccept, isDragReject],
  );

  const IonButton = "ion-button";
  return (
    <div {...getRootProps({ style })}>
      <input {...getInputProps()} />
      {!selected && (
        <p>Drag & drop single image here, or click to select one</p>
      )}
      <IonButton onClick={selected ? () => open() : undefined}>
        {selected ? "Open another image" : "Open image"}
      </IonButton>
      {isDragActive && isDragAccept && <DropOverlay />}
      {isDragActive && isDragReject && <DropRejectOverlay />}
      {selectedImage && <SelectedImage image={selectedImage} />}
    </div>
  );
}

function SelectedImage({ image }: { image: File }) {
  // TODO(memory leak): data_url should be freed.
  const src = useMemo(() => URL.createObjectURL(image), [image]);
  return (
    <div>
      <img
        src={src}
        alt="Selected Image"
        style={{
          width: "100%",
          maxHeight: "300px",
          maxWidth: "300px",
        }}
      />
      <p>{image.name}</p>
    </div>
  );
}

function DropRejectOverlay() {
  return (
    <div
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: "100%",
        height: "100%",
        backgroundColor: "rgba(255, 0, 0, 0.5)",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <p style={{ color: "white" }}>Only images are allowed</p>
    </div>
  );
}

function DropOverlay() {
  return (
    <div
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: "100%",
        height: "100%",
        backgroundColor: "rgba(0, 0, 0, 0.5)",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <p style={{ color: "white" }}>Drop it!</p>
    </div>
  );
}
