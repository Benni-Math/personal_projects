
const Note = ({ noteInput }) => {
  return (
    <div>
      <h1>{noteInput.title}</h1>
      <p>{noteInput.text}</p>
    </div>
  )
}

export default Note