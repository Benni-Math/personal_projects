import { useState } from 'react'

const NameInput = ({ onSubmit }) => {
	const [text, setText] = useState('')

	const addName = (e) => {
		e.preventDefault()

		if (!text) {
			alert('Please type your name!')
			return
		}

		onSubmit(text)

		setText('')
	}

  return (
    <form className='form-control' onSubmit={addName}>
			<div>
        <input 
			type='text' 
			placeholder='Insert Name' 
			value={text} 
			onChange={(e) => setText(e.target.value)}
		/>
        <input type='submit' value='Submit' className='btn btn-block'/>
			</div>
    </form>
  )
}

export default NameInput