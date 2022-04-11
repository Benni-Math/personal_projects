import { useState } from 'react'
import NameInput from './NameInput'
import Button from './Button'


const Header = ({ openForm, name, setName }) => {
	const [switchButton, setSwitchButton] = useState(true)

	const onSubmit = (newName) => {
		setName(newName)
	}

	const onSwitch = () => {
		openForm()
		setSwitchButton(!switchButton)
	}

  return (
		<>
        {name==='' ? (
					<div>
						<h1>What is your name?</h1>
						<NameInput onSubmit={onSubmit}/>
					</div>) : (
					<header className='header'>
						<h1>{`${name}'s Notepad`}</h1>
						<Button 
							color={switchButton ? 'green' : 'red'} 
							text={switchButton ? 'Open' : 'Close'} 
							onClick={onSwitch}/>
					</header>
				)}
		</>        
  )
}

export default Header