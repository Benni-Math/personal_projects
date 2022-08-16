/**
 * Conway's Game of Life
 * @author Benedikt Arnarsson
 */

// Import React
import React, { useReducer, useEffect, useRef } from 'react';

// Import shared helpers
import isValidPoint2D from './shared/helpers/isValidPoint2D';
import numNeighbors2D from './shared/helpers/numNeighbors2D';

// Import shared constants
// import ADD_CONSTANT_NAME from './addConstantFilename';

// Import shared types
import Color from './shared/types/Color';

// Import shared components
import Cell from './shared/components/Cell';
import Grid from './shared/components/Grid';
import Controller from './shared/components/Controller';

// Import style
// import './styles.css';

// TODO: The idea is for the Game to be self-contained, and entirely portable (with 'shared' directory)
// The Controller and Grid will communicate through the Game component,
// which will also control the logic of Conway's Game of Life
// The Cells are just a piece within the Grid
// The Controller is just used as input to change the state within the Game
// The Grid only displays the state which the Game holds
//
// TODO: how should the style be customized in a portable fashion?
// Might need props for that...

/*------------------------------------------------------------------------*/
/*                                Constants                               */
/*------------------------------------------------------------------------*/

// Could make these more 'reactive' or 'versatile'
const CELL_SIZE = 20;
const WIDTH = 800;
const HEIGHT = 600;

/*------------------------------------------------------------------------*/
/*                                  State                                 */
/*------------------------------------------------------------------------*/

// TODO: should I add a dark-mode and light-mode?
// enum View = {
//   Dark: 'dark',
//   Light: 'light'
// }

/* -------- State Definition -------- */

type State = {
  isRunning: boolean,
  interval: number,
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

const GameOfLife: React.FC<Props> = (props) => {
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
export default GameOfLife;
