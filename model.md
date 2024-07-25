@startuml

class Strategy{
    Action act(agentId)
    feedback(agentId , action)
}

class TitForTat{
    blackList 
}

class AlwaysDefect{

}

class AlwaysCooperate{

}

class OnlyBurntOnce{
    blackList 
}

Strategy --|> OnlyBurntOnce    
Strategy --|> TitForTat        
Strategy --|> AlwaysDefect     
Strategy --|> AlwaysCooperate  

class World{
    loop()
}

Agent -- Strategy

class Agent<<Strategy>> {
    Action act(agentId)  
}
note left: The Agent act function forwards to the Strategy act function

Agent -- Action 
Strategy -- Action 


enum Action {
    Cooperate,
    Defect
}

World *-> "*" Agent

@enduml

