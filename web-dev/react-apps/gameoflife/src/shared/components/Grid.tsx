// Planned general Grid component

// .Board {
//   position: relative;
//   margin: 0 auto;
//   background-color: #000;
//   background-image:
//     linear-gradient(#333 1px, transparent 1px),
//     linear-gradient(90deg, #333 1px, transparent 1px);
// }
/**
 * Grid component for simulating and playing games.
 * Depends on the Cell component and Color type.
 * @author Benedikt Arnarsson
 */

// Import React
import React, { useReducer, useEffect, useRef } from 'react';

// Import shared helpers
// import addHelperName from './addHelperFilename';

// Import shared constants
// import ADD_CONSTANT_NAME from './addConstantFilename';

// Import shared types
import Color from '../types/Color';

// Import shared components
import Cell from './Cell';

// Import helpers
// import addHelperName from './addHelperFilename';

// Import constants
// import ADD_CONSTANT_NAME from './addConstantFilename';

// Import types
// import AddTypeName from './AddSharedTypeFilename';

// Import components
// import AddComponentName from './AddComponentFilename';

// Import style
// import './AddNameOfStylesheet.scss';

/*------------------------------------------------------------------------*/
/*                                  Types                                 */
/*------------------------------------------------------------------------*/

// Props definition
type Props = {
  // Height of the grid
  height: number,
  // Width of the grid
  width: number,
  // Cell size
  cellSize: number,
};

/*------------------------------------------------------------------------*/
/*                                Constants                               */
/*------------------------------------------------------------------------*/

// Add description of constant
// const ADD_CONSTANT_NAME = 'add constant value';

/*------------------------------------------------------------------------*/
/*                                  State                                 */
/*------------------------------------------------------------------------*/

/* -------- State Definition -------- */

type State = {
  board: boolean[][],
};

/* ------------- Actions ------------ */

// Types of actions
enum ActionType {
  // Add description of action type
  AddActionTypeName = 'add-action-type-name',
}

// Action definitions
type Action = (
  | {
    // Action type
    type: ActionType.AddActionTypeName,
    // Add description of required payload property
    addPayloadPropertyName: addPayloadPropertyType,
    // Add description of optional payload property
    addPayloadPropertyName?: addPayloadPropertyType,
  }
  | {
    // Action type
    type: (
      | ActionType.AddActionTypeWithNoPayload
      | ActionType.AddActionTypeWithNoPayload
    ),
  }
);

/**
 * Reducer that executes actions
 * @author Add Your Name
 * @param state current state
 * @param action action to execute
 */
const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case ActionType.AddActionType: {
      return {
        ...state,
        addStateVariableName: addStateVariableNewValue,
      };
    }
    default: {
      return state;
    }
  }
};

/*------------------------------------------------------------------------*/
/*                             Static Helpers                             */
/*------------------------------------------------------------------------*/

/**
 * Add description of helper
 * @author Add Your Name
 * @param addArgName add arg description
 * @param addArgName add arg description
 * @returns add return description
 */
const addHelperName = (
  addRequiredArgName: addRequiredArgType,
  addOptionalArgName?: addOptionalArgType,
  addOptionalArgWithDefaultName?: addOptionalArgType = addArgDefault,
): addReturnType => {
  // TODO: implement
};

/*------------------------------------------------------------------------*/
/*                                Component                               */
/*------------------------------------------------------------------------*/

const Grid: React.FC<Props> = (props) => {
  /*------------------------------------------------------------------------*/
  /*                                  Setup                                 */
  /*------------------------------------------------------------------------*/

  /* -------------- Props ------------- */

  // Destructure all props
  const {
    addRequiredPropName,
    addOptionalPropName = 'add default value of prop',
  } = props;

  /* -------------- State ------------- */

  // Initial state
  const initialState: State = {
    addStateVariableName: 'add state variable initial value',
  };

  // Initialize state
  const [state, dispatch] = useReducer(reducer, initialState);

  // Destructure common state
  const {
    addStateVariableName,
    addStateVariableName,
  } = state;

  /* -------------- Refs -------------- */

  // Initialize refs
  const addRefName = useRef<AddRefType>(null);

  /*------------------------------------------------------------------------*/
  /*                           Component Functions                          */
  /*------------------------------------------------------------------------*/

  /**
   * Add component helper function description
   * @author Add Your Name
   * @param addArgName add description of argument
   * @param [addOptionalArgName] add description of optional argument
   * @returns add description of return
   */
  const addComponentHelperFunctionName = (
    addArgName: addArgType,
    addOptionalArgName?: addOptionalArgType,
  ): addReturnType => {
    // TODO: implement
  };

  /*------------------------------------------------------------------------*/
  /*                           Lifecycle Functions                          */
  /*------------------------------------------------------------------------*/

  /**
   * Mount
   * @author Add Your Name
   */
  useEffect(
    () => {
      (async () => {
        // TODO: implement
      })();
    },
    [],
  );

  /**
   * Update (also called on mount)
   * @author Add Your Name
   */
  useEffect(
    () => {
      // TODO: implement
    },
    [addTriggerVariable],
  );

  /**
   * Unmount
   * @author Add Your Name
   */
  useEffect(
    () => {
      return () => {
        // TODO: implement
      };
    },
    [],
  );

  /*------------------------------------------------------------------------*/
  /*                                 Render                                 */
  /*------------------------------------------------------------------------*/

  /*----------------------------------------*/
  /*                  Modal                 */
  /*----------------------------------------*/

  // Modal that may be defined
  let modal: React.ReactNode;

  /* ------- AddFirstTypeOfModal ------ */

  if (addLogicToDetermineIfModalIsVisible) {
    // TODO: implement

    // Create modal
    modal = (
      <addJSXOfModal />
    );
  }

  /*----------------------------------------*/
  /*                 Views                  */
  /*----------------------------------------*/

  // Body that will be filled with the current view
  let body: React.ReactNode;

  /* -------- AddFirstViewName -------- */

  if (view === View.AddViewName) {
    // TODO: implement

    // Create body
    body = (
      <addJSXOfBody />
    );
  }

  /* -------- AddSecondViewName -------- */

  if (view === View.AddViewName) {
    // TODO: implement

    // Create body
    body = (
      <addJSXOfBody />
    );
  }

  /*----------------------------------------*/
  /*                 Main UI                */
  /*----------------------------------------*/

  return (
    <addContainersForBody>
      {/* Add Modal */}
      {modal}

      {/* Add Body */}
      {body}
    </addContainersForBody>
  );
};

/*------------------------------------------------------------------------*/
/*                                 Wrap Up                                */
/*------------------------------------------------------------------------*/

// Export component
export default Grid;
