__doc__: str

def load_standard() -> None:
    """
    Loads the geometry of a standard field
    """


def load_dropshot() -> None:
    """
    Loads the geometry of a standard dropshot field
    """


def load_hoops() -> None:
    """
    Loads the geometry of a standard hoops field
    """


def load_standard_throwback() -> None:
    """
    Loads the geometry of the field Throwback Stadium
    """


def load_standard_heatseeker() -> None:
    """
    Loads the geometry of a standard field for Heatseeker
    """


try:
    from rlbot_flatbuffers import GameTickPacket
except ImportError:
    pass


def tick(packet: GameTickPacket) -> None:
    """
    Parses the game tick packet from RLBot for information about the ball & world gravity
    """


class BallSlice:
    time: float
    location: tuple[float, float, float]
    velocity: tuple[float, float, float]
    angular_velocity: tuple[float, float, float]

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


class BallPredictionStruct:
    num_slices: int
    slices: list[BallSlice]

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


def step_ball() -> BallSlice:
    """
    Steps the ball forward 1/120th of a second

    Returns the information about the new state of the ball
    """


def get_ball_prediction_struct_full() -> BallPredictionStruct:
    """
    Equivalent to calling `step_ball()` 720 times (6 seconds)

    This function is preferred because there is no overhead from Python

    Returns all details about every step of the ball, including the angular velocity
    """


def get_ball_prediction_struct_for_time_full(time: float) -> BallPredictionStruct:
    """
    Equivalent to calling `step_ball()` 120 * `time` times

    This function is preferred because there is no overhead from Python

    Returns all details about every step of the ball, including the angular velocity

    time: The seconds into the future that the ball path prediction should be generated
    """


class HalfBallSlice:
    time: float
    location: tuple[float, float, float]
    velocity: tuple[float, float, float]

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


class HalfBallPredictionStruct:
    num_slices: int
    slices: list[HalfBallSlice]

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


def get_ball_prediction_struct() -> HalfBallPredictionStruct:
    """
    Equivalent to calling `step_ball()` 720 times (6 seconds)

    This function is preferred because there is no overhead from Python, and returns in a format similar to the RLBot framework

    Returns details about every other step of the ball
    """


def get_ball_prediction_struct_for_time(time: float) -> HalfBallPredictionStruct:
    """
    Equivalent to calling `step_ball()` 120 * `time` times

    This function is preferred because there is no overhead from Python, and returns in a format similar to the RLBot framework

    Returns details about every other step of the ball

    time: The seconds into the future that the ball path prediction should be generated
    """
